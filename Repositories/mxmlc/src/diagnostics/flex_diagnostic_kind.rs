#[repr(i32)]
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum FlexDiagnosticKind {
    EntityIsNotAType = 2048,
    ImplicitCoercionToUnrelatedType = 2049,
    EntityIsReadOnly = 2050,
    EntityIsWriteOnly = 2051,
    EntityMustNotBeDeleted = 2052,
    UndefinedProperty = 2053,
    AmbiguousReference = 2054,
    AccessOfVoid = 2055,
    AccessOfNullable = 2056,
    CouldNotExpandInlineConstant = 2057,
    ReachedMaximumCycles = 2058,
    NullNotExpectedHere = 2059,
    CouldNotParseNumber = 2060,
    NoMatchingEnumMember = 2061,
    UnexpectedThis = 2062,
    ArrayLengthNotEqualsTupleLength = 2063,
    UnexpectedElision = 2064,
    UnexpectedArray = 2065,
    UnexpectedRest = 2066,
    UnexpectedObject = 2067,
    DynamicOptionNotSupported = 2068,
    UnknownOptionForClass = 2069,
    MustSpecifyOption = 2070,
    UnexpectedFieldName = 2071,
    UnexpectedNewBase = 2072,
    IncorrectNumArguments = 2073,
    IncorrectNumArgumentsNoMoreThan = 2074,
    UndefinedPropertyWithStaticType = 2075,
    InapplicableFilter = 2076,
    InapplicableDescendants = 2077,
    ASuperExpCanBeUsedOnlyIn = 2078,
    ASuperExpCanOnlyBeUsedInSubclasses = 2079,
    CallOnArrayType = 2080,
    CallOnNonFunction = 2081,
    NonParameterizedType = 2082,
    AwaitOperandMustBeAPromise = 2083,
    OperandMustBeNumber = 2084,
    ReferenceIsAlreadyNonNullable = 2085,
    YieldIsNotSupported = 2086,
    UnrelatedMathOperation = 2087,
    ComparisonBetweenUnrelatedTypes = 2088,
    UnrelatedTernaryOperands = 2089,
    SystemNamespaceNotFound = 2090,
    RestParameterMustBeArray = 2091,
    AConflictExistsWithDefinition = 2092,
    DuplicateVariableDefinition = 2093,
    DuplicateClassDefinition = 2094,
    DuplicateInterfaceDefinition = 2095,
    DuplicateFunctionDefinition = 2096,
    UnexpectedFieldNameInDestructuring = 2097,
    EntityIsNotAConstant = 2098,
    ReturnValueHasNoTypeDeclaration = 2099,
    ReturnTypeDeclarationMustBePromise = 2100,
    ReturnTypeInferenceIsNotImplemented = 2101,
    NanComparison = 2102,
    NotABooleanConstant = 2103,
    EmptyPackage = 2104,
    ImportOfUndefined = 2105,
    NotANamespaceConstant = 2106,
    CannotResolveConfigConstant = 2107,
    ConcatenatingSelfReferentialPackage = 2108,
    CallOnDateType = 2109,
    AccessControlNamespaceNotAllowedHere = 2110,
    CannotUseDestructuringHere = 2111,
    ShadowingDefinitionInBaseClass = 2112,
    VariableHasNoTypeAnnotation = 2113,
    ConstantMustContainInitializer = 2114,
    ExternalFunctionMustBeNativeOrAbstract = 2115,
}

impl FlexDiagnosticKind {
    pub fn id(&self) -> i32 {
        *self as i32
    }
}