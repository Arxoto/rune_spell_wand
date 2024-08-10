#![allow(dead_code)]

use std::ops;

/// 可修正 wand spell  
/// 使用者被法杖修正 环境被法术效果修正  
/// 符文基础消耗 增益 或是法杖自带属性 均由此实现
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Attribute {
    /// 消耗符文抽取数
    rune_num: i32,
    /// 初始耗时 第一次施法所需准备时间
    init_time: i64,
    /// 回正耗时 每次施法的间隔 直接在当前时刻加上时长 下次施放时检查时刻即可
    back_time: i64,
    /// 轮次冷却 每轮施法的间隔 注意轮次间不需要加上初始耗时
    round_time: i64,
    /// 轮次 一轮允许有几次施法
    round_num: i32,
    /// 偏转 投射物的初始方向偏移角度 越小越好 负数取零
    deflection: f32,
    /// 体能 stamina
    stamina_cost: f32,
    /// 平衡 balance
    balance_cost: f32,
    /// 法力 magicka
    magicka_cost: f32,
    /// 发射力道 影响投射物的初始速度
    shoot_force: f32,
    /// 投射物的锋利度修正
    bullet_sharp: f32,
    /// 投射物存在时间
    bullet_lifetime: i64,
    // spell

    // /// 威能提升
    // power: f32,
}

impl ops::AddAssign for Attribute {
    fn add_assign(&mut self, rhs: Self) {
        self.rune_num += rhs.rune_num;
        self.init_time += rhs.init_time;
        self.back_time += rhs.back_time;
        self.round_time += rhs.round_time;
        self.round_num += rhs.round_num;
        self.deflection += rhs.deflection;
        self.stamina_cost += rhs.stamina_cost;
        self.balance_cost += rhs.balance_cost;
        self.magicka_cost += rhs.magicka_cost;
        self.shoot_force += rhs.shoot_force;
    }
}

/// 被可修改对象实现
pub trait Modifiable {
    fn modify(&mut self, attr: Attribute);
}

/// 可感知 user wand enemy environment
pub trait Veiwable {
    /// 获取环境元素丰度
    fn env_element_abundance(&self) -> Option<(i32, i32, i32)> {
        None
    }
}

pub struct Bullet {
    /// 偏转 投射物的初始方向偏移角度 越小越好 负数取零
    deflection: f32,
    /// 自带的初始速度
    velocity: f32,
    /// 重量 与发射力道一起影响初始速度 与速度一起影响造成的冲击伤害 对投射物来说冲击伤害是相互的
    weight: f32,
    /// 锋利 与速度一起影响造成的切割伤害
    sharp: f32,
    /// 投射物发射后至多存在的时长 自然消亡不会引发触发机制
    lifetime: i64,
    /// 是否具有实体 参与碰撞
    entity: bool,
    /// 受冲击后受损上限 毁坏会引起触发
    health: f32,
}

impl Modifiable for Bullet {
    fn modify(&mut self, attr: Attribute) {
        self.deflection += attr.deflection;
        if self.weight > 0. && attr.shoot_force > 0. {
            self.velocity += attr.shoot_force / self.weight;
        }
        self.sharp += attr.bullet_sharp;
        self.lifetime += attr.bullet_lifetime;
    }
}

// 基于投射物状态触发会导致复杂度增加 暂不做 可以实现投射物速度达到一定程度自动触发
pub struct Trigger {
    iter_limit: i32,
    trigger_attr: Attribute,
    delay_trigger: i64,
}

/// 原初符文 只存在于编译期 法术中的符文应该只有名词符文并内置触发法术
pub enum Rune {
    /// 状态加持 及时生效 有连续施法、法术回复、威能提升等
    Stat(Attribute),
    /// 谓词 修改前一个名词符文的定义 及时生效
    Verb(Attribute, Attribute),
    /// 名词 投射物
    Noun(Attribute, Bullet),
    /// 触发器 将其后符文串打包成一法杖 附加在前一名词上 定时触发或是碰撞触发
    With(Attribute, Trigger),
    /// 标签 对于同一标签 一个符文流中必定存在零或多个  
    /// 其中有且仅有一个标签后不跟流程控制  
    /// 其余标签后必须跟随流程控制  
    Label(Attribute, String),
    /// 流程控制 必定跟在标签之后 满足特定条件就会强制移动法杖指针 可循环实现数量倍增
    Process(
        Attribute,
        fn(context: &dyn Veiwable, current_loop: i32, label_loop: i32) -> bool,
    ),
    /// 复合型 法杖编译后会被展开
    Package(Vec<Rune>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut source = Attribute::default();
        let target = Attribute {
            rune_num: 1,
            init_time: 1,
            back_time: 1,
            round_time: 1,
            round_num: 1,
            deflection: 1.0,
            stamina_cost: 1.0,
            balance_cost: 1.0,
            magicka_cost: 1.0,
            shoot_force: 1.0,
            bullet_sharp: 1.0,
            bullet_lifetime: 1,
        };

        source += target;

        assert_eq!(source, target);
    }
}
