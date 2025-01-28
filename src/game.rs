use std::fmt::Debug;

use console_utils::input::{reveal, select};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

use crate::TIME_BETWEEN;

/// The general Entity type.
///
/// Every in game living thing is an entity: The Player and the Enemies.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Entity {
    name: String,
    life_points: usize,
    dexterity: usize,
    strength: usize,
    weapon: Option<Weapon>,
}

impl Entity {
    pub fn new(
        name: String,
        life_points: usize,
        dexterity: usize,
        strength: usize,
        weapon: Option<Weapon>,
    ) -> Self {
        Self {
            name,
            life_points,
            dexterity,
            strength,
            weapon,
        }
    }

    pub fn apply_dmg(&mut self, dmg: usize) -> bool {
        self.life_points = self.life_points.saturating_sub(dmg);
        self.life_points == 0
    }
}

/// Everything which should be able to fight, needs to implement this trait.
///
/// The trait name `Combatant` is from ChatGPT.
pub trait Combatant {
    /// Gets a reference of the entity.
    fn entity(&self) -> &Entity;

    /// Gets a mutable reference of the entity.
    fn entity_mut(&mut self) -> &mut Entity;

    /// Determine Attack Damage. This function has a default implementation
    /// which can be overwritten (Polymorphism).
    fn attack_damage(&self) -> usize {
        let entity = self.entity();
        if let Some(weapon) = &entity.weapon {
            weapon.calc_damage() + entity.strength
        } else {
            entity.strength
        }
    }

    /// Attacks the `enemy` and subtracts the applied damage to it.
    /// Returns true if enemy is defeated!
    fn attack<E: Combatant>(&mut self, enemy: &mut E) -> bool {
        let self_dmg = self.attack_damage();
        let self_entity = self.entity();
        let enemy_entity = enemy.entity_mut();
        if enemy_entity.apply_dmg(self_dmg) {
            reveal(
                &format!(
                    "Attacke von `{}` hat `{}` besiegt!\n",
                    &self_entity.name, &enemy_entity.name
                ),
                TIME_BETWEEN,
            );
            true
        } else {
            reveal(
                &format!(
                    "Attacke von `{}` hat mit einem Schaden von {} getroffen!\n",
                    &self_entity.name, self_dmg
                ),
                TIME_BETWEEN,
            );
            false
        }
    }

    /// Selector for what the combatant want to do next.
    /// Default is that the `Combatant` can either attack of flee!
    ///
    /// Returns `true` if the enemy is dead or fleeing was successful!
    fn select_action<E: Combatant>(&mut self, enemy: &mut E, game_rules: &mut GameRules) -> bool {
        let attack_dmg = self.attack_damage();
        let n = game_rules.dice.n;
        let options: [&str; 2] = [
            &format!("Angreifen ({attack_dmg} Lebenspunkte Schaden)"),
            &format!("Fliehen (1/{n} Chance)"),
        ];
        let i = select("Aktion auswählen (Pfeiltasten, Enter)", &options);

        match options[i] {
            option if option.starts_with("Angreifen") => self.attack(enemy),
            option if option.starts_with("Fliehen") => {
                let success = game_rules.dice.throw_dice();
                if success {
                    reveal("Fliehen war erfolgreich!\n", TIME_BETWEEN);
                } else {
                    reveal("Fliehen war nicht erfolgreich!\n", TIME_BETWEEN);
                }
                success
            }
            _ => unimplemented!(),
        }
    }

    /// Simulates a fight against an `enemy` with a set of `game_rules`.
    /// Runs until `self` or `enemy` is dead (has 0 `life_points`).
    fn fight<E: Combatant>(&mut self, enemy: &mut E, game_rules: &mut GameRules)
    where
        Self: Sized,
    {
        // Determine fight order; Enemy has constant dexterity; the initiator of the fight, `self`, has to roll
        let ordering = if game_rules.dice.apply_dice_roll(self.entity().dexterity)
            > enemy.entity().dexterity
        {
            Ordering::Player
        } else {
            Ordering::Enemy
        };

        reveal(
            &format!("{ordering:?} wird zuerst angreifen!\n"),
            TIME_BETWEEN,
        );

        // Fight until one is dead
        let mut i = 0;
        loop {
            reveal(&format!("Runde {} hat begonnen!\n", i + 1,), TIME_BETWEEN);
            i += 1;

            reveal(
                &format!(
                    "`{}` hat {} Lebenspunkte und `{}` hat {} Lebenspunkte!\n",
                    self.entity().name,
                    self.entity().life_points,
                    enemy.entity().name,
                    enemy.entity().life_points
                ),
                TIME_BETWEEN,
            );

            match ordering {
                Ordering::Player => {
                    if self.select_action(enemy, game_rules) {
                        break;
                    }
                    if enemy.select_action(self, game_rules) {
                        break;
                    }
                }
                Ordering::Enemy => {
                    if enemy.select_action(self, game_rules) {
                        break;
                    }
                    if self.select_action(enemy, game_rules) {
                        break;
                    }
                }
            }
        }
    }
}

/// General Game Rules.
pub struct GameRules {
    dice: Dice,
}

impl GameRules {
    pub fn new(difficulty: Difficulty) -> Self {
        Self {
            dice: Dice::new(difficulty.to_dice_n()),
        }
    }
}

/// Dice with `n` sides.
///
/// In rust, there are no random functions in it's `std`-library.
/// Therefore using the `rngs`-crate for that!
struct Dice {
    n: usize,
    rng: SmallRng,
}

impl Dice {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            rng: SmallRng::from_os_rng(),
        }
    }

    /// Applys a dice roll to a number by calculating
    /// `(random_range(0..=n) / n) * n` and returning the result.
    pub fn apply_dice_roll(&mut self, num: usize) -> usize {
        let n = self.n;
        ((self.rng.random_range(1..=n) as f64 / n as f64) * num as f64).floor() as usize
    }

    /// Returns true if dice rolled `n`
    pub fn throw_dice(&mut self) -> bool {
        let n = self.n;
        self.rng.random_range(1..=n) == n
    }
}

/// Difficulty used for setting up Game Rules and Dice sides.
#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    /// Dice changes to 1/3
    Easy,
    /// Dice changes to 1/6
    Normal,
    /// Dice changes to 1/9
    Hard,
}

impl Difficulty {
    /// Returns `Difficulty` from `i`. i has to be 0 <= i <= 2 otherwise this function panics!
    pub fn from_i(i: usize) -> Self {
        match i {
            0 => Self::Easy,
            1 => Self::Normal,
            2 => Self::Hard,
            _ => unreachable!(),
        }
    }
    /// Converts the current difficulty to the count of dice sides.
    pub fn to_dice_n(&self) -> usize {
        match self {
            Self::Easy => 3,
            Self::Normal => 6,
            Self::Hard => 9,
        }
    }
}

/// A mage (player) with the option to heal themselves.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Mage {
    pub entity: Entity,
    magic_power: usize,
}

impl Combatant for Mage {
    fn entity(&self) -> &Entity {
        &self.entity
    }

    fn entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }

    /// Overwriting the default implementation for `select_action` by adding an heal option.
    fn select_action<E: Combatant>(&mut self, enemy: &mut E, game_rules: &mut GameRules) -> bool {
        let attack_dmg = self.attack_damage();
        let heal_lp = self.get_heal_lp();
        let n = game_rules.dice.n;
        let options: [&str; 3] = [
            &format!("Angreifen ({attack_dmg} Lebenspunkte Schaden)"),
            &format!("Selber heilen ({heal_lp} Lebenspunkte)"),
            &format!("Fliehen (1/{n} Chance)"),
        ];
        let i = select("Aktion auswählen (Pfeiltasten, Enter)", &options);

        match options[i] {
            option if option.starts_with("Angreifen") => self.attack(enemy),
            option if option.starts_with("Selber heilen") => {
                self.heal();
                false
            }
            option if option.starts_with("Fliehen") => {
                let success = game_rules.dice.throw_dice();
                if success {
                    reveal("Fliehen war erfolgreich!\n", TIME_BETWEEN);
                } else {
                    reveal("Fliehen war nicht erfolgreich!\n", TIME_BETWEEN);
                }
                success
            }
            _ => unimplemented!(),
        }
    }
}

impl Mage {
    pub fn new(entity: Entity, magic_power: usize) -> Self {
        Self {
            entity,
            magic_power,
        }
    }

    /// Calculates the heal lp and returns it.
    pub fn get_heal_lp(&self) -> usize {
        let weapon_power = if let Some(weapon) = &self.entity.weapon {
            weapon.spell_power
        } else {
            0
        };
        self.magic_power * weapon_power
    }

    /// Applys the heal of the mage to it's own health.
    pub fn heal(&mut self) {
        let heal_lp = self.get_heal_lp();
        self.entity.life_points += heal_lp;
        reveal(
            &format!(
                "`{}` hat sich mit {} Lebenspunkten geheilt!\n",
                self.entity.name, heal_lp
            ),
            TIME_BETWEEN,
        )
    }
}

/// A fighter (player) with extra endurance which strengthens their attack damage.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Fighter {
    pub entity: Entity,
    endurance: usize,
}

impl Combatant for Fighter {
    fn entity(&self) -> &Entity {
        &self.entity
    }

    fn entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }

    /// Overwriting the default implementation for `attack_damage` by adding an endurance multiplier.
    fn attack_damage(&self) -> usize {
        let entity = self.entity();
        let norm_attack = if let Some(weapon) = &entity.weapon {
            weapon.calc_damage() + entity.strength
        } else {
            entity.strength
        };
        norm_attack * self.endurance
    }
}

impl Fighter {
    pub fn new(entity: Entity, endurance: usize) -> Self {
        Self { entity, endurance }
    }
}

/// A monster struct which the player fights against.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Monster {
    pub entity: Entity,
}

impl Combatant for Monster {
    fn entity(&self) -> &Entity {
        &self.entity
    }

    fn entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }

    /// Overwriting the default implementation for `select_action` by removing all options.
    /// A monster will always attack.
    fn select_action<E: Combatant>(&mut self, enemy: &mut E, _game_rules: &mut GameRules) -> bool {
        self.attack(enemy)
    }
}

impl Monster {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

/// Weapon can have different material and a spell power (if seen as a staff).
#[derive(Debug, Serialize, Deserialize)]
pub struct Weapon {
    material: Material,
    pub spell_power: usize,
}

impl Weapon {
    pub fn new(material: Material, spell_power: usize) -> Self {
        Self {
            material,
            spell_power,
        }
    }

    /// Calculate damage modifier of the weapon.
    pub fn calc_damage(&self) -> usize {
        self.material.calc_modifier() + self.spell_power
    }
}

// Material of the weapon. `Wood` is the weakest and `Diamond` the strongest material.
#[repr(usize)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Material {
    Wood = 1,
    Stone,
    Iron,
    Gold,
    MagicOre,
    Diamond,
}

impl Material {
    // Calculating the material modifier. Used for damage calculation.
    pub fn calc_modifier(&self) -> usize {
        *self as usize
    }
}

/// Fight order.
enum Ordering {
    Player,
    Enemy,
}

impl Debug for Ordering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Player => write!(f, "Spieler"),
            Self::Enemy => write!(f, "Gegner"),
        }
    }
}
