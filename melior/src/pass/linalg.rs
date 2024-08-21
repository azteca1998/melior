//! Linalg passes.

melior_macro::passes!(
    "Linalg",
    [
        mlirCreateLinalgConvertElementwiseToLinalgPass,
        // mlirCreateLinalgLinalgBufferizePass,
        mlirCreateLinalgLinalgDetensorizePass,
        mlirCreateLinalgLinalgElementwiseOpFusionPass,
        mlirCreateLinalgLinalgFoldUnitExtentDimsPass,
        // mlirCreateLinalgLinalgGeneralizationPass,
        mlirCreateLinalgLinalgInlineScalarOperandsPass,
        // mlirCreateLinalgLinalgLowerToAffineLoopsPass,
        // mlirCreateLinalgLinalgLowerToLoopsPass,
        // mlirCreateLinalgLinalgLowerToParallelLoopsPass,
        mlirCreateLinalgLinalgNamedOpConversionPass,
    ]
);
