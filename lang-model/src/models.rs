#![allow(non_camel_case_types)]
use tree_sitter::Node;
#[allow(clippy::result_unit_err)]
pub fn type_tree<'a>(
    tree: &'a tree_sitter::Tree,
    source_code: &'a str,
) -> Result<source_file<'a>, ()> {
    use tree_sitter::{Query, QueryCursor};
    let mut query_cursor = QueryCursor::new();
    let error_query = Query::new(tree_sitter_mumps::language(), "(ERROR)").unwrap();
    let errors = query_cursor.matches(&error_query, tree.root_node(), source_code.as_bytes());
    if errors.count() != 0 {
        Err(())
    } else {
        Ok(source_file::create(tree.root_node()))
    }
}
#[derive(Clone)]
pub struct Ascii<'a> {
    node: Node<'a>,
}
impl<'a> Ascii<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> Ascii<'a> {
    pub fn args(&self) -> Vec<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.collect()
    }
}
#[derive(Clone)]
pub struct BinaryExpression<'a> {
    node: Node<'a>,
}
impl<'a> BinaryExpression<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> BinaryExpression<'a> {
    pub fn exp_right(&self) -> Expression<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("exp_right", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.next().unwrap()
    }
}
impl<'a> BinaryExpression<'a> {
    pub fn opp(&self) -> BinaryOpp<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("opp", &mut cursur)
            .filter(|x| x.is_named())
            .map(BinaryOpp::create);
        children.next().unwrap()
    }
}
impl<'a> BinaryExpression<'a> {
    pub fn exp_left(&self) -> Expression<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("exp_left", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct BinaryOpp<'a> {
    node: Node<'a>,
}
impl<'a> BinaryOpp<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub enum BinaryOppChildren<'a> {
    OPADD(OPADD<'a>),
    OPAND(OPAND<'a>),
    OPCAT(OPCAT<'a>),
    OPCON(OPCON<'a>),
    OPDIV(OPDIV<'a>),
    OPEQL(OPEQL<'a>),
    OPFOL(OPFOL<'a>),
    OPGTR(OPGTR<'a>),
    OPINT(OPINT<'a>),
    OPLES(OPLES<'a>),
    OPMOD(OPMOD<'a>),
    OPMUL(OPMUL<'a>),
    OPNAND(OPNAND<'a>),
    OPNCON(OPNCON<'a>),
    OPNEQL(OPNEQL<'a>),
    OPNFOL(OPNFOL<'a>),
    OPNGTR(OPNGTR<'a>),
    OPNLES(OPNLES<'a>),
    OPNSAF(OPNSAF<'a>),
    OPPOW(OPPOW<'a>),
    OPSAF(OPSAF<'a>),
    OPSUB(OPSUB<'a>),
}
impl<'a> BinaryOppChildren<'a> {
    fn create(node: Node<'a>) -> Self {
        match node.kind() {
            "OPADD" => Self::OPADD(OPADD::create(node)),
            "OPAND" => Self::OPAND(OPAND::create(node)),
            "OPCAT" => Self::OPCAT(OPCAT::create(node)),
            "OPCON" => Self::OPCON(OPCON::create(node)),
            "OPDIV" => Self::OPDIV(OPDIV::create(node)),
            "OPEQL" => Self::OPEQL(OPEQL::create(node)),
            "OPFOL" => Self::OPFOL(OPFOL::create(node)),
            "OPGTR" => Self::OPGTR(OPGTR::create(node)),
            "OPINT" => Self::OPINT(OPINT::create(node)),
            "OPLES" => Self::OPLES(OPLES::create(node)),
            "OPMOD" => Self::OPMOD(OPMOD::create(node)),
            "OPMUL" => Self::OPMUL(OPMUL::create(node)),
            "OPNAND" => Self::OPNAND(OPNAND::create(node)),
            "OPNCON" => Self::OPNCON(OPNCON::create(node)),
            "OPNEQL" => Self::OPNEQL(OPNEQL::create(node)),
            "OPNFOL" => Self::OPNFOL(OPNFOL::create(node)),
            "OPNGTR" => Self::OPNGTR(OPNGTR::create(node)),
            "OPNLES" => Self::OPNLES(OPNLES::create(node)),
            "OPNSAF" => Self::OPNSAF(OPNSAF::create(node)),
            "OPPOW" => Self::OPPOW(OPPOW::create(node)),
            "OPSAF" => Self::OPSAF(OPSAF::create(node)),
            "OPSUB" => Self::OPSUB(OPSUB::create(node)),
            _ => unreachable!(),
        }
    }
}
impl<'a> BinaryOpp<'a> {
    pub fn children(&self) -> BinaryOppChildren<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .named_children(&mut cursur)
            .map(BinaryOppChildren::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct Block<'a> {
    node: Node<'a>,
}
impl<'a> Block<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub enum BlockChildren<'a> {
    Block(Block<'a>),
    line(line<'a>),
}
impl<'a> BlockChildren<'a> {
    fn create(node: Node<'a>) -> Self {
        match node.kind() {
            "Block" => Self::Block(Block::create(node)),
            "line" => Self::line(line::create(node)),
            _ => unreachable!(),
        }
    }
}
impl<'a> Block<'a> {
    pub fn children(&self) -> Vec<BlockChildren<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .named_children(&mut cursur)
            .map(BlockChildren::create);
        children.collect()
    }
}
#[derive(Clone)]
pub struct Brake<'a> {
    node: Node<'a>,
}
impl<'a> Brake<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct BrakeCommand<'a> {
    node: Node<'a>,
}
impl<'a> BrakeCommand<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> BrakeCommand<'a> {
    pub fn children(&self) -> Brake<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self.node.named_children(&mut cursur).map(Brake::create);
        children.next().unwrap()
    }
}
impl<'a> BrakeCommand<'a> {
    pub fn args(&self) -> Vec<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.collect()
    }
}
impl<'a> BrakeCommand<'a> {
    pub fn post_condition(&self) -> Option<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("post_condition", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.next()
    }
}
#[derive(Clone)]
pub struct ByRef<'a> {
    node: Node<'a>,
}
impl<'a> ByRef<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> ByRef<'a> {
    pub fn children(&self) -> Variable<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self.node.named_children(&mut cursur).map(Variable::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct Char<'a> {
    node: Node<'a>,
}
impl<'a> Char<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> Char<'a> {
    pub fn args(&self) -> Vec<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.collect()
    }
}
#[derive(Clone)]
pub struct Clear<'a> {
    node: Node<'a>,
}
impl<'a> Clear<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Close<'a> {
    node: Node<'a>,
}
impl<'a> Close<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct CloseCommand<'a> {
    node: Node<'a>,
}
impl<'a> CloseCommand<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> CloseCommand<'a> {
    pub fn children(&self) -> Close<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self.node.named_children(&mut cursur).map(Close::create);
        children.next().unwrap()
    }
}
impl<'a> CloseCommand<'a> {
    pub fn post_condition(&self) -> Option<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("post_condition", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.next()
    }
}
impl<'a> CloseCommand<'a> {
    pub fn args(&self) -> Vec<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.collect()
    }
}
#[derive(Clone)]
pub struct Compress<'a> {
    node: Node<'a>,
}
impl<'a> Compress<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Data<'a> {
    node: Node<'a>,
}
impl<'a> Data<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> Data<'a> {
    pub fn var(&self) -> Variable<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("var", &mut cursur)
            .filter(|x| x.is_named())
            .map(Variable::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct Device<'a> {
    node: Node<'a>,
}
impl<'a> Device<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Directory<'a> {
    node: Node<'a>,
}
impl<'a> Directory<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Do<'a> {
    node: Node<'a>,
}
impl<'a> Do<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct DoArg<'a> {
    node: Node<'a>,
}
impl<'a> DoArg<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> DoArg<'a> {
    pub fn post_condition(&self) -> Option<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("post_condition", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.next()
    }
}
impl<'a> DoArg<'a> {
    pub fn function(&self) -> ExtrinsicFunction<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("function", &mut cursur)
            .filter(|x| x.is_named())
            .map(ExtrinsicFunction::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct DoCommand<'a> {
    node: Node<'a>,
}
impl<'a> DoCommand<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> DoCommand<'a> {
    pub fn children(&self) -> Do<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self.node.named_children(&mut cursur).map(Do::create);
        children.next().unwrap()
    }
}
impl<'a> DoCommand<'a> {
    pub fn args(&self) -> Vec<DoArg<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(DoArg::create);
        children.collect()
    }
}
impl<'a> DoCommand<'a> {
    pub fn post_condition(&self) -> Option<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("post_condition", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.next()
    }
}
#[derive(Clone)]
pub struct Ecode<'a> {
    node: Node<'a>,
}
impl<'a> Ecode<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Else<'a> {
    node: Node<'a>,
}
impl<'a> Else<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct ElseCommand<'a> {
    node: Node<'a>,
}
impl<'a> ElseCommand<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> ElseCommand<'a> {
    pub fn children(&self) -> Else<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self.node.named_children(&mut cursur).map(Else::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct ErrMsg<'a> {
    node: Node<'a>,
}
impl<'a> ErrMsg<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Estack<'a> {
    node: Node<'a>,
}
impl<'a> Estack<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Etrap<'a> {
    node: Node<'a>,
}
impl<'a> Etrap<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct ExpFunctions<'a> {
    node: Node<'a>,
}
impl<'a> ExpFunctions<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub enum ExpFunctionsChildren<'a> {
    Ascii(Ascii<'a>),
    Char(Char<'a>),
    Extract(Extract<'a>),
    Find(Find<'a>),
    Fnumber(Fnumber<'a>),
    Justify(Justify<'a>),
    Length(Length<'a>),
    Piece(Piece<'a>),
    Random(Random<'a>),
    Reverse(Reverse<'a>),
    Stack(Stack<'a>),
    Text(Text<'a>),
    Translate(Translate<'a>),
    View(View<'a>),
}
impl<'a> ExpFunctionsChildren<'a> {
    fn create(node: Node<'a>) -> Self {
        match node.kind() {
            "Ascii" => Self::Ascii(Ascii::create(node)),
            "Char" => Self::Char(Char::create(node)),
            "Extract" => Self::Extract(Extract::create(node)),
            "Find" => Self::Find(Find::create(node)),
            "Fnumber" => Self::Fnumber(Fnumber::create(node)),
            "Justify" => Self::Justify(Justify::create(node)),
            "Length" => Self::Length(Length::create(node)),
            "Piece" => Self::Piece(Piece::create(node)),
            "Random" => Self::Random(Random::create(node)),
            "Reverse" => Self::Reverse(Reverse::create(node)),
            "Stack" => Self::Stack(Stack::create(node)),
            "Text" => Self::Text(Text::create(node)),
            "Translate" => Self::Translate(Translate::create(node)),
            "View" => Self::View(View::create(node)),
            _ => unreachable!(),
        }
    }
}
impl<'a> ExpFunctions<'a> {
    pub fn children(&self) -> ExpFunctionsChildren<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .named_children(&mut cursur)
            .map(ExpFunctionsChildren::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct Expression<'a> {
    node: Node<'a>,
}
impl<'a> Expression<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub enum ExpressionChildren<'a> {
    BinaryExpression(BinaryExpression<'a>),
    Expression(Expression<'a>),
    ExtrinsicFunction(ExtrinsicFunction<'a>),
    InderectExpression(InderectExpression<'a>),
    IntrinsicFunction(IntrinsicFunction<'a>),
    IntrinsicVar(IntrinsicVar<'a>),
    PaternMatchExpression(PaternMatchExpression<'a>),
    UnaryExpression(UnaryExpression<'a>),
    Variable(Variable<'a>),
    XCall(XCall<'a>),
    number(number<'a>),
    string(string<'a>),
}
impl<'a> ExpressionChildren<'a> {
    fn create(node: Node<'a>) -> Self {
        match node.kind() {
            "BinaryExpression" => Self::BinaryExpression(BinaryExpression::create(node)),
            "Expression" => Self::Expression(Expression::create(node)),
            "ExtrinsicFunction" => Self::ExtrinsicFunction(ExtrinsicFunction::create(node)),
            "InderectExpression" => Self::InderectExpression(InderectExpression::create(node)),
            "IntrinsicFunction" => Self::IntrinsicFunction(IntrinsicFunction::create(node)),
            "IntrinsicVar" => Self::IntrinsicVar(IntrinsicVar::create(node)),
            "PaternMatchExpression" => {
                Self::PaternMatchExpression(PaternMatchExpression::create(node))
            }
            "UnaryExpression" => Self::UnaryExpression(UnaryExpression::create(node)),
            "Variable" => Self::Variable(Variable::create(node)),
            "XCall" => Self::XCall(XCall::create(node)),
            "number" => Self::number(number::create(node)),
            "string" => Self::string(string::create(node)),
            _ => unreachable!(),
        }
    }
}
impl<'a> Expression<'a> {
    pub fn children(&self) -> ExpressionChildren<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .named_children(&mut cursur)
            .map(ExpressionChildren::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct Extract<'a> {
    node: Node<'a>,
}
impl<'a> Extract<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> Extract<'a> {
    pub fn args(&self) -> Vec<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.collect()
    }
}
#[derive(Clone)]
pub struct ExtrinsicFunction<'a> {
    node: Node<'a>,
}
impl<'a> ExtrinsicFunction<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> ExtrinsicFunction<'a> {
    pub fn routine(&self) -> Option<identifier<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("routine", &mut cursur)
            .filter(|x| x.is_named())
            .map(identifier::create);
        children.next()
    }
}
#[derive(Clone)]
pub enum ExtrinsicFunctionArgs<'a> {
    ByRef(ByRef<'a>),
    Expression(Expression<'a>),
    VarUndefined(VarUndefined<'a>),
}
impl<'a> ExtrinsicFunctionArgs<'a> {
    fn create(node: Node<'a>) -> Self {
        match node.kind() {
            "ByRef" => Self::ByRef(ByRef::create(node)),
            "Expression" => Self::Expression(Expression::create(node)),
            "VarUndefined" => Self::VarUndefined(VarUndefined::create(node)),
            _ => unreachable!(),
        }
    }
}
impl<'a> ExtrinsicFunction<'a> {
    pub fn args(&self) -> Vec<ExtrinsicFunctionArgs<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(ExtrinsicFunctionArgs::create);
        children.collect()
    }
}
impl<'a> ExtrinsicFunction<'a> {
    pub fn tag(&self) -> Option<TagName<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("tag", &mut cursur)
            .filter(|x| x.is_named())
            .map(TagName::create);
        children.next()
    }
}
#[derive(Clone)]
pub struct File<'a> {
    node: Node<'a>,
}
impl<'a> File<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Find<'a> {
    node: Node<'a>,
}
impl<'a> Find<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> Find<'a> {
    pub fn args(&self) -> Vec<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.collect()
    }
}
#[derive(Clone)]
pub struct Fnumber<'a> {
    node: Node<'a>,
}
impl<'a> Fnumber<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> Fnumber<'a> {
    pub fn args(&self) -> Vec<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.collect()
    }
}
#[derive(Clone)]
pub struct For<'a> {
    node: Node<'a>,
}
impl<'a> For<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> For<'a> {
    pub fn variable(&self) -> Option<Variable<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("variable", &mut cursur)
            .filter(|x| x.is_named())
            .map(Variable::create);
        children.next()
    }
}
impl<'a> For<'a> {
    pub fn args(&self) -> Vec<ForArg<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(ForArg::create);
        children.collect()
    }
}
#[derive(Clone)]
pub struct ForArg<'a> {
    node: Node<'a>,
}
impl<'a> ForArg<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> ForArg<'a> {
    pub fn children(&self) -> Vec<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .named_children(&mut cursur)
            .map(Expression::create);
        children.collect()
    }
}
#[derive(Clone)]
pub struct Fork<'a> {
    node: Node<'a>,
}
impl<'a> Fork<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Get<'a> {
    node: Node<'a>,
}
impl<'a> Get<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> Get<'a> {
    pub fn args(&self) -> Option<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.next()
    }
}
impl<'a> Get<'a> {
    pub fn var(&self) -> Variable<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("var", &mut cursur)
            .filter(|x| x.is_named())
            .map(Variable::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct GetEnv<'a> {
    node: Node<'a>,
}
impl<'a> GetEnv<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct GlobalUciEnvVariable<'a> {
    node: Node<'a>,
}
impl<'a> GlobalUciEnvVariable<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> GlobalUciEnvVariable<'a> {
    pub fn children(&self) -> Vec<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .named_children(&mut cursur)
            .map(Expression::create);
        children.collect()
    }
}
#[derive(Clone)]
pub struct GlobalUciVariable<'a> {
    node: Node<'a>,
}
impl<'a> GlobalUciVariable<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> GlobalUciVariable<'a> {
    pub fn children(&self) -> Expression<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .named_children(&mut cursur)
            .map(Expression::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct GlobalVariable<'a> {
    node: Node<'a>,
}
impl<'a> GlobalVariable<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Horolog<'a> {
    node: Node<'a>,
}
impl<'a> Horolog<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Host<'a> {
    node: Node<'a>,
}
impl<'a> Host<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct IC<'a> {
    node: Node<'a>,
}
impl<'a> IC<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Increment<'a> {
    node: Node<'a>,
}
impl<'a> Increment<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> Increment<'a> {
    pub fn var(&self) -> Variable<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("var", &mut cursur)
            .filter(|x| x.is_named())
            .map(Variable::create);
        children.next().unwrap()
    }
}
impl<'a> Increment<'a> {
    pub fn args(&self) -> Option<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.next()
    }
}
#[derive(Clone)]
pub struct InderectExpression<'a> {
    node: Node<'a>,
}
impl<'a> InderectExpression<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> InderectExpression<'a> {
    pub fn children(&self) -> Expression<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .named_children(&mut cursur)
            .map(Expression::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct IndirectVariable<'a> {
    node: Node<'a>,
}
impl<'a> IndirectVariable<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> IndirectVariable<'a> {
    pub fn children(&self) -> Expression<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .named_children(&mut cursur)
            .map(Expression::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct IntrinsicFunction<'a> {
    node: Node<'a>,
}
impl<'a> IntrinsicFunction<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub enum IntrinsicFunctionChildren<'a> {
    ExpFunctions(ExpFunctions<'a>),
    Select(Select<'a>),
    VarFunctions(VarFunctions<'a>),
}
impl<'a> IntrinsicFunctionChildren<'a> {
    fn create(node: Node<'a>) -> Self {
        match node.kind() {
            "ExpFunctions" => Self::ExpFunctions(ExpFunctions::create(node)),
            "Select" => Self::Select(Select::create(node)),
            "VarFunctions" => Self::VarFunctions(VarFunctions::create(node)),
            _ => unreachable!(),
        }
    }
}
impl<'a> IntrinsicFunction<'a> {
    pub fn children(&self) -> IntrinsicFunctionChildren<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .named_children(&mut cursur)
            .map(IntrinsicFunctionChildren::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct IntrinsicVar<'a> {
    node: Node<'a>,
}
impl<'a> IntrinsicVar<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub enum IntrinsicVarChildren<'a> {
    Device(Device<'a>),
    Ecode(Ecode<'a>),
    Estack(Estack<'a>),
    Etrap(Etrap<'a>),
    Horolog(Horolog<'a>),
    Io(Io<'a>),
    Job(Job<'a>),
    Key(Key<'a>),
    Principal(Principal<'a>),
    Quit(Quit<'a>),
    Reference(Reference<'a>),
    StackVar(StackVar<'a>),
    Storage(Storage<'a>),
    System(System<'a>),
    Test(Test<'a>),
    X(X<'a>),
    Y(Y<'a>),
}
impl<'a> IntrinsicVarChildren<'a> {
    fn create(node: Node<'a>) -> Self {
        match node.kind() {
            "Device" => Self::Device(Device::create(node)),
            "Ecode" => Self::Ecode(Ecode::create(node)),
            "Estack" => Self::Estack(Estack::create(node)),
            "Etrap" => Self::Etrap(Etrap::create(node)),
            "Horolog" => Self::Horolog(Horolog::create(node)),
            "Io" => Self::Io(Io::create(node)),
            "Job" => Self::Job(Job::create(node)),
            "Key" => Self::Key(Key::create(node)),
            "Principal" => Self::Principal(Principal::create(node)),
            "Quit" => Self::Quit(Quit::create(node)),
            "Reference" => Self::Reference(Reference::create(node)),
            "StackVar" => Self::StackVar(StackVar::create(node)),
            "Storage" => Self::Storage(Storage::create(node)),
            "System" => Self::System(System::create(node)),
            "Test" => Self::Test(Test::create(node)),
            "X" => Self::X(X::create(node)),
            "Y" => Self::Y(Y::create(node)),
            _ => unreachable!(),
        }
    }
}
impl<'a> IntrinsicVar<'a> {
    pub fn children(&self) -> IntrinsicVarChildren<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .named_children(&mut cursur)
            .map(IntrinsicVarChildren::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct Io<'a> {
    node: Node<'a>,
}
impl<'a> Io<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Job<'a> {
    node: Node<'a>,
}
impl<'a> Job<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Justify<'a> {
    node: Node<'a>,
}
impl<'a> Justify<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> Justify<'a> {
    pub fn args(&self) -> Vec<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.collect()
    }
}
#[derive(Clone)]
pub struct Key<'a> {
    node: Node<'a>,
}
impl<'a> Key<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Length<'a> {
    node: Node<'a>,
}
impl<'a> Length<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> Length<'a> {
    pub fn args(&self) -> Vec<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.collect()
    }
}
#[derive(Clone)]
pub struct NakedVariable<'a> {
    node: Node<'a>,
}
impl<'a> NakedVariable<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Name<'a> {
    node: Node<'a>,
}
impl<'a> Name<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> Name<'a> {
    pub fn args(&self) -> Option<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.next()
    }
}
impl<'a> Name<'a> {
    pub fn var(&self) -> Variable<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("var", &mut cursur)
            .filter(|x| x.is_named())
            .map(Variable::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct New<'a> {
    node: Node<'a>,
}
impl<'a> New<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct NewCommand<'a> {
    node: Node<'a>,
}
impl<'a> NewCommand<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> NewCommand<'a> {
    pub fn children(&self) -> New<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self.node.named_children(&mut cursur).map(New::create);
        children.next().unwrap()
    }
}
impl<'a> NewCommand<'a> {
    pub fn post_condition(&self) -> Option<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("post_condition", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.next()
    }
}
impl<'a> NewCommand<'a> {
    pub fn args(&self) -> Vec<identifier<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(identifier::create);
        children.collect()
    }
}
#[derive(Clone)]
pub struct Next<'a> {
    node: Node<'a>,
}
impl<'a> Next<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> Next<'a> {
    pub fn var(&self) -> Variable<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("var", &mut cursur)
            .filter(|x| x.is_named())
            .map(Variable::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct OPADD<'a> {
    node: Node<'a>,
}
impl<'a> OPADD<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct OPEQL<'a> {
    node: Node<'a>,
}
impl<'a> OPEQL<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct OPFOL<'a> {
    node: Node<'a>,
}
impl<'a> OPFOL<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct OPMINUS<'a> {
    node: Node<'a>,
}
impl<'a> OPMINUS<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct OPMOD<'a> {
    node: Node<'a>,
}
impl<'a> OPMOD<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct OPNOT<'a> {
    node: Node<'a>,
}
impl<'a> OPNOT<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct OPPAT<'a> {
    node: Node<'a>,
}
impl<'a> OPPAT<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct OPPLUS<'a> {
    node: Node<'a>,
}
impl<'a> OPPLUS<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct OPSUB<'a> {
    node: Node<'a>,
}
impl<'a> OPSUB<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct OpCom<'a> {
    node: Node<'a>,
}
impl<'a> OpCom<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Order<'a> {
    node: Node<'a>,
}
impl<'a> Order<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> Order<'a> {
    pub fn args(&self) -> Option<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.next()
    }
}
impl<'a> Order<'a> {
    pub fn var(&self) -> Variable<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("var", &mut cursur)
            .filter(|x| x.is_named())
            .map(Variable::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct Patern<'a> {
    node: Node<'a>,
}
impl<'a> Patern<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub enum PaternChildren<'a> {
    PaternElement(PaternElement<'a>),
    PaternRepetitionCount(PaternRepetitionCount<'a>),
}
impl<'a> PaternChildren<'a> {
    fn create(node: Node<'a>) -> Self {
        match node.kind() {
            "PaternElement" => Self::PaternElement(PaternElement::create(node)),
            "PaternRepetitionCount" => {
                Self::PaternRepetitionCount(PaternRepetitionCount::create(node))
            }
            _ => unreachable!(),
        }
    }
}
impl<'a> Patern<'a> {
    pub fn children(&self) -> Vec<PaternChildren<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .named_children(&mut cursur)
            .map(PaternChildren::create);
        children.collect()
    }
}
#[derive(Clone)]
pub struct PaternElement<'a> {
    node: Node<'a>,
}
impl<'a> PaternElement<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub enum PaternElementChildren<'a> {
    Patern(Patern<'a>),
    string(string<'a>),
}
impl<'a> PaternElementChildren<'a> {
    fn create(node: Node<'a>) -> Self {
        match node.kind() {
            "Patern" => Self::Patern(Patern::create(node)),
            "string" => Self::string(string::create(node)),
            _ => unreachable!(),
        }
    }
}
impl<'a> PaternElement<'a> {
    pub fn children(&self) -> Vec<PaternElementChildren<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .named_children(&mut cursur)
            .map(PaternElementChildren::create);
        children.collect()
    }
}
#[derive(Clone)]
pub struct PaternMatchExpression<'a> {
    node: Node<'a>,
}
impl<'a> PaternMatchExpression<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub enum PaternMatchExpressionExp_right<'a> {
    Expression(Expression<'a>),
    Patern(Patern<'a>),
}
impl<'a> PaternMatchExpressionExp_right<'a> {
    fn create(node: Node<'a>) -> Self {
        match node.kind() {
            "Expression" => Self::Expression(Expression::create(node)),
            "Patern" => Self::Patern(Patern::create(node)),
            _ => unreachable!(),
        }
    }
}
impl<'a> PaternMatchExpression<'a> {
    pub fn exp_right(&self) -> PaternMatchExpressionExp_right<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("exp_right", &mut cursur)
            .filter(|x| x.is_named())
            .map(PaternMatchExpressionExp_right::create);
        children.next().unwrap()
    }
}
impl<'a> PaternMatchExpression<'a> {
    pub fn exp_left(&self) -> Expression<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("exp_left", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.next().unwrap()
    }
}
impl<'a> PaternMatchExpression<'a> {
    pub fn opp(&self) -> PatternOpp<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("opp", &mut cursur)
            .filter(|x| x.is_named())
            .map(PatternOpp::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct PaternRepetitionCount<'a> {
    node: Node<'a>,
}
impl<'a> PaternRepetitionCount<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct PatternOpp<'a> {
    node: Node<'a>,
}
impl<'a> PatternOpp<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub enum PatternOppChildren<'a> {
    OPNPAT(OPNPAT<'a>),
    OPPAT(OPPAT<'a>),
}
impl<'a> PatternOppChildren<'a> {
    fn create(node: Node<'a>) -> Self {
        match node.kind() {
            "OPNPAT" => Self::OPNPAT(OPNPAT::create(node)),
            "OPPAT" => Self::OPPAT(OPPAT::create(node)),
            _ => unreachable!(),
        }
    }
}
impl<'a> PatternOpp<'a> {
    pub fn children(&self) -> PatternOppChildren<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .named_children(&mut cursur)
            .map(PatternOppChildren::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct Piece<'a> {
    node: Node<'a>,
}
impl<'a> Piece<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> Piece<'a> {
    pub fn args(&self) -> Vec<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.collect()
    }
}
#[derive(Clone)]
pub struct Principal<'a> {
    node: Node<'a>,
}
impl<'a> Principal<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct QUIT<'a> {
    node: Node<'a>,
}
impl<'a> QUIT<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct QUITCommand<'a> {
    node: Node<'a>,
}
impl<'a> QUITCommand<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> QUITCommand<'a> {
    pub fn children(&self) -> QUIT<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self.node.named_children(&mut cursur).map(QUIT::create);
        children.next().unwrap()
    }
}
impl<'a> QUITCommand<'a> {
    pub fn args(&self) -> Vec<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.collect()
    }
}
impl<'a> QUITCommand<'a> {
    pub fn post_condition(&self) -> Option<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("post_condition", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.next()
    }
}
#[derive(Clone)]
pub struct Qlength<'a> {
    node: Node<'a>,
}
impl<'a> Qlength<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> Qlength<'a> {
    pub fn var(&self) -> Variable<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("var", &mut cursur)
            .filter(|x| x.is_named())
            .map(Variable::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct Qsubscript<'a> {
    node: Node<'a>,
}
impl<'a> Qsubscript<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> Qsubscript<'a> {
    pub fn var(&self) -> Variable<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("var", &mut cursur)
            .filter(|x| x.is_named())
            .map(Variable::create);
        children.next().unwrap()
    }
}
impl<'a> Qsubscript<'a> {
    pub fn args(&self) -> Expression<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct Query<'a> {
    node: Node<'a>,
}
impl<'a> Query<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> Query<'a> {
    pub fn args(&self) -> Option<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.next()
    }
}
impl<'a> Query<'a> {
    pub fn var(&self) -> Variable<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("var", &mut cursur)
            .filter(|x| x.is_named())
            .map(Variable::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct Quit<'a> {
    node: Node<'a>,
}
impl<'a> Quit<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Random<'a> {
    node: Node<'a>,
}
impl<'a> Random<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> Random<'a> {
    pub fn args(&self) -> Expression<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct Reference<'a> {
    node: Node<'a>,
}
impl<'a> Reference<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Reverse<'a> {
    node: Node<'a>,
}
impl<'a> Reverse<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> Reverse<'a> {
    pub fn args(&self) -> Expression<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct RouChk<'a> {
    node: Node<'a>,
}
impl<'a> RouChk<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Select<'a> {
    node: Node<'a>,
}
impl<'a> Select<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> Select<'a> {
    pub fn children(&self) -> Vec<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .named_children(&mut cursur)
            .map(Expression::create);
        children.collect()
    }
}
#[derive(Clone)]
pub struct SetEnv<'a> {
    node: Node<'a>,
}
impl<'a> SetEnv<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Signal<'a> {
    node: Node<'a>,
}
impl<'a> Signal<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Spawn<'a> {
    node: Node<'a>,
}
impl<'a> Spawn<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Stack<'a> {
    node: Node<'a>,
}
impl<'a> Stack<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> Stack<'a> {
    pub fn args(&self) -> Vec<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.collect()
    }
}
#[derive(Clone)]
pub struct StackVar<'a> {
    node: Node<'a>,
}
impl<'a> StackVar<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Storage<'a> {
    node: Node<'a>,
}
impl<'a> Storage<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct System<'a> {
    node: Node<'a>,
}
impl<'a> System<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Tab<'a> {
    node: Node<'a>,
}
impl<'a> Tab<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> Tab<'a> {
    pub fn children(&self) -> Expression<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .named_children(&mut cursur)
            .map(Expression::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct Tag<'a> {
    node: Node<'a>,
}
impl<'a> Tag<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> Tag<'a> {
    pub fn name(&self) -> TagName<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("name", &mut cursur)
            .filter(|x| x.is_named())
            .map(TagName::create);
        children.next().unwrap()
    }
}
impl<'a> Tag<'a> {
    pub fn block(&self) -> Option<Block<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("block", &mut cursur)
            .filter(|x| x.is_named())
            .map(Block::create);
        children.next()
    }
}
#[derive(Clone)]
pub struct TagName<'a> {
    node: Node<'a>,
}
impl<'a> TagName<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub enum TagNameChildren<'a> {
    NumericIdentifier(NumericIdentifier<'a>),
    identifier(identifier<'a>),
}
impl<'a> TagNameChildren<'a> {
    fn create(node: Node<'a>) -> Self {
        match node.kind() {
            "NumericIdentifier" => Self::NumericIdentifier(NumericIdentifier::create(node)),
            "identifier" => Self::identifier(identifier::create(node)),
            _ => unreachable!(),
        }
    }
}
impl<'a> TagName<'a> {
    pub fn children(&self) -> TagNameChildren<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .named_children(&mut cursur)
            .map(TagNameChildren::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct Test<'a> {
    node: Node<'a>,
}
impl<'a> Test<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Text<'a> {
    node: Node<'a>,
}
impl<'a> Text<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> Text<'a> {
    pub fn args(&self) -> Expression<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct Translate<'a> {
    node: Node<'a>,
}
impl<'a> Translate<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> Translate<'a> {
    pub fn args(&self) -> Vec<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.collect()
    }
}
#[derive(Clone)]
pub struct UnaryExpression<'a> {
    node: Node<'a>,
}
impl<'a> UnaryExpression<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> UnaryExpression<'a> {
    pub fn opp(&self) -> UnaryOpp<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("opp", &mut cursur)
            .filter(|x| x.is_named())
            .map(UnaryOpp::create);
        children.next().unwrap()
    }
}
impl<'a> UnaryExpression<'a> {
    pub fn exp(&self) -> Expression<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("exp", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct UnaryOpp<'a> {
    node: Node<'a>,
}
impl<'a> UnaryOpp<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub enum UnaryOppChildren<'a> {
    OPMINUS(OPMINUS<'a>),
    OPNOT(OPNOT<'a>),
    OPPLUS(OPPLUS<'a>),
}
impl<'a> UnaryOppChildren<'a> {
    fn create(node: Node<'a>) -> Self {
        match node.kind() {
            "OPMINUS" => Self::OPMINUS(OPMINUS::create(node)),
            "OPNOT" => Self::OPNOT(OPNOT::create(node)),
            "OPPLUS" => Self::OPPLUS(OPPLUS::create(node)),
            _ => unreachable!(),
        }
    }
}
impl<'a> UnaryOpp<'a> {
    pub fn children(&self) -> UnaryOppChildren<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .named_children(&mut cursur)
            .map(UnaryOppChildren::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct VarFunctions<'a> {
    node: Node<'a>,
}
impl<'a> VarFunctions<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub enum VarFunctionsChildren<'a> {
    Data(Data<'a>),
    Get(Get<'a>),
    Increment(Increment<'a>),
    Name(Name<'a>),
    Next(Next<'a>),
    Order(Order<'a>),
    Qlength(Qlength<'a>),
    Qsubscript(Qsubscript<'a>),
    Query(Query<'a>),
}
impl<'a> VarFunctionsChildren<'a> {
    fn create(node: Node<'a>) -> Self {
        match node.kind() {
            "Data" => Self::Data(Data::create(node)),
            "Get" => Self::Get(Get::create(node)),
            "Increment" => Self::Increment(Increment::create(node)),
            "Name" => Self::Name(Name::create(node)),
            "Next" => Self::Next(Next::create(node)),
            "Order" => Self::Order(Order::create(node)),
            "Qlength" => Self::Qlength(Qlength::create(node)),
            "Qsubscript" => Self::Qsubscript(Qsubscript::create(node)),
            "Query" => Self::Query(Query::create(node)),
            _ => unreachable!(),
        }
    }
}
impl<'a> VarFunctions<'a> {
    pub fn children(&self) -> VarFunctionsChildren<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .named_children(&mut cursur)
            .map(VarFunctionsChildren::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct Variable<'a> {
    node: Node<'a>,
}
impl<'a> Variable<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> Variable<'a> {
    pub fn subs(&self) -> Vec<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("subs", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.collect()
    }
}
impl<'a> Variable<'a> {
    pub fn name(&self) -> Option<identifier<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("name", &mut cursur)
            .filter(|x| x.is_named())
            .map(identifier::create);
        children.next()
    }
}
#[derive(Clone)]
pub enum VariableHeading<'a> {
    GlobalUciEnvVariable(GlobalUciEnvVariable<'a>),
    GlobalUciVariable(GlobalUciVariable<'a>),
    GlobalVariable(GlobalVariable<'a>),
    IndirectVariable(IndirectVariable<'a>),
    NakedVariable(NakedVariable<'a>),
}
impl<'a> VariableHeading<'a> {
    fn create(node: Node<'a>) -> Self {
        match node.kind() {
            "GlobalUciEnvVariable" => {
                Self::GlobalUciEnvVariable(GlobalUciEnvVariable::create(node))
            }
            "GlobalUciVariable" => Self::GlobalUciVariable(GlobalUciVariable::create(node)),
            "GlobalVariable" => Self::GlobalVariable(GlobalVariable::create(node)),
            "IndirectVariable" => Self::IndirectVariable(IndirectVariable::create(node)),
            "NakedVariable" => Self::NakedVariable(NakedVariable::create(node)),
            _ => unreachable!(),
        }
    }
}
impl<'a> Variable<'a> {
    pub fn heading(&self) -> Option<VariableHeading<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("heading", &mut cursur)
            .filter(|x| x.is_named())
            .map(VariableHeading::create);
        children.next()
    }
}
#[derive(Clone)]
pub struct Version<'a> {
    node: Node<'a>,
}
impl<'a> Version<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct View<'a> {
    node: Node<'a>,
}
impl<'a> View<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> View<'a> {
    pub fn args(&self) -> Vec<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.collect()
    }
}
#[derive(Clone)]
pub struct Wait<'a> {
    node: Node<'a>,
}
impl<'a> Wait<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Write<'a> {
    node: Node<'a>,
}
impl<'a> Write<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct WriteArg<'a> {
    node: Node<'a>,
}
impl<'a> WriteArg<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub enum WriteArgChildren<'a> {
    Bang(Bang<'a>),
    Clear(Clear<'a>),
    Expression(Expression<'a>),
    Tab(Tab<'a>),
}
impl<'a> WriteArgChildren<'a> {
    fn create(node: Node<'a>) -> Self {
        match node.kind() {
            "Bang" => Self::Bang(Bang::create(node)),
            "Clear" => Self::Clear(Clear::create(node)),
            "Expression" => Self::Expression(Expression::create(node)),
            "Tab" => Self::Tab(Tab::create(node)),
            _ => unreachable!(),
        }
    }
}
impl<'a> WriteArg<'a> {
    pub fn children(&self) -> WriteArgChildren<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .named_children(&mut cursur)
            .map(WriteArgChildren::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct WriteCommand<'a> {
    node: Node<'a>,
}
impl<'a> WriteCommand<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> WriteCommand<'a> {
    pub fn children(&self) -> Write<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self.node.named_children(&mut cursur).map(Write::create);
        children.next().unwrap()
    }
}
impl<'a> WriteCommand<'a> {
    pub fn args(&self) -> Vec<WriteArg<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(WriteArg::create);
        children.collect()
    }
}
impl<'a> WriteCommand<'a> {
    pub fn post_condition(&self) -> Option<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("post_condition", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.next()
    }
}
#[derive(Clone)]
pub struct X<'a> {
    node: Node<'a>,
}
impl<'a> X<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct XCall<'a> {
    node: Node<'a>,
}
impl<'a> XCall<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub enum XCallCode<'a> {
    Compress(Compress<'a>),
    Debug(Debug<'a>),
    Directory(Directory<'a>),
    E(E<'a>),
    ErrMsg(ErrMsg<'a>),
    File(File<'a>),
    Fork(Fork<'a>),
    GetEnv(GetEnv<'a>),
    Host(Host<'a>),
    IC(IC<'a>),
    OpCom(OpCom<'a>),
    Paschk(Paschk<'a>),
    RouChk(RouChk<'a>),
    SetEnv(SetEnv<'a>),
    Signal(Signal<'a>),
    Spawn(Spawn<'a>),
    V(V<'a>),
    Version(Version<'a>),
    Wait(Wait<'a>),
    XCallX(XCallX<'a>),
    Xrsm(Xrsm<'a>),
    Zwrite(Zwrite<'a>),
}
impl<'a> XCallCode<'a> {
    fn create(node: Node<'a>) -> Self {
        match node.kind() {
            "Compress" => Self::Compress(Compress::create(node)),
            "Debug" => Self::Debug(Debug::create(node)),
            "Directory" => Self::Directory(Directory::create(node)),
            "E" => Self::E(E::create(node)),
            "ErrMsg" => Self::ErrMsg(ErrMsg::create(node)),
            "File" => Self::File(File::create(node)),
            "Fork" => Self::Fork(Fork::create(node)),
            "GetEnv" => Self::GetEnv(GetEnv::create(node)),
            "Host" => Self::Host(Host::create(node)),
            "IC" => Self::IC(IC::create(node)),
            "OpCom" => Self::OpCom(OpCom::create(node)),
            "Paschk" => Self::Paschk(Paschk::create(node)),
            "RouChk" => Self::RouChk(RouChk::create(node)),
            "SetEnv" => Self::SetEnv(SetEnv::create(node)),
            "Signal" => Self::Signal(Signal::create(node)),
            "Spawn" => Self::Spawn(Spawn::create(node)),
            "V" => Self::V(V::create(node)),
            "Version" => Self::Version(Version::create(node)),
            "Wait" => Self::Wait(Wait::create(node)),
            "XCallX" => Self::XCallX(XCallX::create(node)),
            "Xrsm" => Self::Xrsm(Xrsm::create(node)),
            "Zwrite" => Self::Zwrite(Zwrite::create(node)),
            _ => unreachable!(),
        }
    }
}
impl<'a> XCall<'a> {
    pub fn code(&self) -> XCallCode<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("code", &mut cursur)
            .filter(|x| x.is_named())
            .map(XCallCode::create);
        children.next().unwrap()
    }
}
impl<'a> XCall<'a> {
    pub fn args(&self) -> Vec<Expression<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .children_by_field_name("args", &mut cursur)
            .filter(|x| x.is_named())
            .map(Expression::create);
        children.collect()
    }
}
#[derive(Clone)]
pub struct Y<'a> {
    node: Node<'a>,
}
impl<'a> Y<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Zwrite<'a> {
    node: Node<'a>,
}
impl<'a> Zwrite<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct command<'a> {
    node: Node<'a>,
}
impl<'a> command<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub enum commandChildren<'a> {
    BrakeCommand(BrakeCommand<'a>),
    CloseCommand(CloseCommand<'a>),
    DoCommand(DoCommand<'a>),
    ElseCommand(ElseCommand<'a>),
    For(For<'a>),
    NewCommand(NewCommand<'a>),
    QUITCommand(QUITCommand<'a>),
    WriteCommand(WriteCommand<'a>),
}
impl<'a> commandChildren<'a> {
    fn create(node: Node<'a>) -> Self {
        match node.kind() {
            "BrakeCommand" => Self::BrakeCommand(BrakeCommand::create(node)),
            "CloseCommand" => Self::CloseCommand(CloseCommand::create(node)),
            "DoCommand" => Self::DoCommand(DoCommand::create(node)),
            "ElseCommand" => Self::ElseCommand(ElseCommand::create(node)),
            "For" => Self::For(For::create(node)),
            "NewCommand" => Self::NewCommand(NewCommand::create(node)),
            "QUITCommand" => Self::QUITCommand(QUITCommand::create(node)),
            "WriteCommand" => Self::WriteCommand(WriteCommand::create(node)),
            _ => unreachable!(),
        }
    }
}
impl<'a> command<'a> {
    pub fn children(&self) -> commandChildren<'a> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self
            .node
            .named_children(&mut cursur)
            .map(commandChildren::create);
        children.next().unwrap()
    }
}
#[derive(Clone)]
pub struct line<'a> {
    node: Node<'a>,
}
impl<'a> line<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> line<'a> {
    pub fn children(&self) -> Vec<command<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self.node.named_children(&mut cursur).map(command::create);
        children.collect()
    }
}
#[derive(Clone)]
pub struct number<'a> {
    node: Node<'a>,
}
impl<'a> number<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct source_file<'a> {
    node: Node<'a>,
}
impl<'a> source_file<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
impl<'a> source_file<'a> {
    pub fn children(&self) -> Vec<Tag<'a>> {
        let mut cursur = self.node.walk();
        #[allow(unused_mut)]
        let mut children = self.node.named_children(&mut cursur).map(Tag::create);
        children.collect()
    }
}
#[derive(Clone)]
pub struct Bang<'a> {
    node: Node<'a>,
}
impl<'a> Bang<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Debug<'a> {
    node: Node<'a>,
}
impl<'a> Debug<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct E<'a> {
    node: Node<'a>,
}
impl<'a> E<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct NumericIdentifier<'a> {
    node: Node<'a>,
}
impl<'a> NumericIdentifier<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct OPAND<'a> {
    node: Node<'a>,
}
impl<'a> OPAND<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct OPCAT<'a> {
    node: Node<'a>,
}
impl<'a> OPCAT<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct OPCON<'a> {
    node: Node<'a>,
}
impl<'a> OPCON<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct OPDIV<'a> {
    node: Node<'a>,
}
impl<'a> OPDIV<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct OPGTR<'a> {
    node: Node<'a>,
}
impl<'a> OPGTR<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct OPINT<'a> {
    node: Node<'a>,
}
impl<'a> OPINT<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct OPLES<'a> {
    node: Node<'a>,
}
impl<'a> OPLES<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct OPMUL<'a> {
    node: Node<'a>,
}
impl<'a> OPMUL<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct OPNAND<'a> {
    node: Node<'a>,
}
impl<'a> OPNAND<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct OPNCON<'a> {
    node: Node<'a>,
}
impl<'a> OPNCON<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct OPNEQL<'a> {
    node: Node<'a>,
}
impl<'a> OPNEQL<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct OPNFOL<'a> {
    node: Node<'a>,
}
impl<'a> OPNFOL<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct OPNGTR<'a> {
    node: Node<'a>,
}
impl<'a> OPNGTR<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct OPNLES<'a> {
    node: Node<'a>,
}
impl<'a> OPNLES<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct OPNPAT<'a> {
    node: Node<'a>,
}
impl<'a> OPNPAT<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct OPNSAF<'a> {
    node: Node<'a>,
}
impl<'a> OPNSAF<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct OPPOW<'a> {
    node: Node<'a>,
}
impl<'a> OPPOW<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct OPSAF<'a> {
    node: Node<'a>,
}
impl<'a> OPSAF<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Paschk<'a> {
    node: Node<'a>,
}
impl<'a> Paschk<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct V<'a> {
    node: Node<'a>,
}
impl<'a> V<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct VarUndefined<'a> {
    node: Node<'a>,
}
impl<'a> VarUndefined<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct XCallX<'a> {
    node: Node<'a>,
}
impl<'a> XCallX<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct Xrsm<'a> {
    node: Node<'a>,
}
impl<'a> Xrsm<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct identifier<'a> {
    node: Node<'a>,
}
impl<'a> identifier<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
#[derive(Clone)]
pub struct string<'a> {
    node: Node<'a>,
}
impl<'a> string<'a> {
    fn create(node: Node<'a>) -> Self {
        Self { node }
    }
    pub fn node(&self) -> &Node<'a> {
        &self.node
    }
}
