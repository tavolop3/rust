; ModuleID = 'autocfg_7717dfebf98f5077_1.4ec65df31a355831-cgu.0'
source_filename = "autocfg_7717dfebf98f5077_1.4ec65df31a355831-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@alloc_f93507f8ba4b5780b14b2c2584609be0 = private unnamed_addr constant <{ [8 x i8] }> <{ [8 x i8] c"\00\00\00\00\00\00\F0?" }>, align 8
@alloc_ef0a1f828f3393ef691f2705e817091c = private unnamed_addr constant <{ [8 x i8] }> <{ [8 x i8] c"\00\00\00\00\00\00\00@" }>, align 8

; core::f64::<impl f64>::total_cmp
; Function Attrs: inlinehint nonlazybind uwtable
define hidden i8 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$9total_cmp17h9b59c9762e38673bE"(ptr align 8 %self, ptr align 8 %other) unnamed_addr #0 !dbg !17 {
start:
  %other.dbg.spill = alloca [8 x i8], align 8
  %self.dbg.spill = alloca [8 x i8], align 8
  %_6 = alloca [8 x i8], align 8
  %_3 = alloca [8 x i8], align 8
  store ptr %self, ptr %self.dbg.spill, align 8
    #dbg_declare(ptr %self.dbg.spill, !26, !DIExpression(), !29)
  store ptr %other, ptr %other.dbg.spill, align 8
    #dbg_declare(ptr %other.dbg.spill, !27, !DIExpression(), !30)
  %_5 = load double, ptr %self, align 8, !dbg !31
  %_4 = bitcast double %_5 to i64, !dbg !32
  store i64 %_4, ptr %_3, align 8, !dbg !31
  %_8 = load double, ptr %other, align 8, !dbg !38
  %_7 = bitcast double %_8 to i64, !dbg !39
  store i64 %_7, ptr %_6, align 8, !dbg !38
  %_13 = load i64, ptr %_3, align 8, !dbg !41
  %_12 = ashr i64 %_13, 63, !dbg !42
  %_10 = lshr i64 %_12, 1, !dbg !43
  %0 = load i64, ptr %_3, align 8, !dbg !44
  %1 = xor i64 %0, %_10, !dbg !44
  store i64 %1, ptr %_3, align 8, !dbg !44
  %_18 = load i64, ptr %_6, align 8, !dbg !45
  %_17 = ashr i64 %_18, 63, !dbg !46
  %_15 = lshr i64 %_17, 1, !dbg !47
  %2 = load i64, ptr %_6, align 8, !dbg !48
  %3 = xor i64 %2, %_15, !dbg !48
  store i64 %3, ptr %_6, align 8, !dbg !48
  %_19 = load i64, ptr %_3, align 8, !dbg !49
  %_20 = load i64, ptr %_6, align 8, !dbg !59
  %4 = icmp sgt i64 %_19, %_20, !dbg !60
  %5 = zext i1 %4 to i8, !dbg !60
  %6 = icmp slt i64 %_19, %_20, !dbg !60
  %7 = zext i1 %6 to i8, !dbg !60
  %_0 = sub nsw i8 %5, %7, !dbg !60
  ret i8 %_0, !dbg !61
}

; autocfg_7717dfebf98f5077_1::probe
; Function Attrs: nonlazybind uwtable
define void @_ZN26autocfg_7717dfebf98f5077_15probe17hc733925f1f19f1ceE() unnamed_addr #1 !dbg !62 {
start:
; call core::f64::<impl f64>::total_cmp
  %_1 = call i8 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$9total_cmp17h9b59c9762e38673bE"(ptr align 8 @alloc_f93507f8ba4b5780b14b2c2584609be0, ptr align 8 @alloc_ef0a1f828f3393ef691f2705e817091c), !dbg !67
  ret void, !dbg !68
}

attributes #0 = { inlinehint nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }

!llvm.module.flags = !{!0, !1, !2, !3}
!llvm.ident = !{!4}
!llvm.dbg.cu = !{!5}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{i32 2, !"Dwarf Version", i32 4}
!3 = !{i32 2, !"Debug Info Version", i32 3}
!4 = !{!"rustc version 1.85.1 (4eb161250 2025-03-15)"}
!5 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !6, producer: "clang LLVM (rustc version 1.85.1 (4eb161250 2025-03-15))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, enums: !7, splitDebugInlining: false, nameTableKind: None)
!6 = !DIFile(filename: "autocfg_7717dfebf98f5077_1/@/autocfg_7717dfebf98f5077_1.4ec65df31a355831-cgu.0", directory: "/home/tao/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/num-traits-0.2.19")
!7 = !{!8}
!8 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "Ordering", scope: !10, file: !9, baseType: !12, size: 8, align: 8, flags: DIFlagEnumClass, elements: !13)
!9 = !DIFile(filename: "<unknown>", directory: "")
!10 = !DINamespace(name: "cmp", scope: !11)
!11 = !DINamespace(name: "core", scope: null)
!12 = !DIBasicType(name: "i8", size: 8, encoding: DW_ATE_signed)
!13 = !{!14, !15, !16}
!14 = !DIEnumerator(name: "Less", value: -1)
!15 = !DIEnumerator(name: "Equal", value: 0)
!16 = !DIEnumerator(name: "Greater", value: 1)
!17 = distinct !DISubprogram(name: "total_cmp", linkageName: "_ZN4core3f6421_$LT$impl$u20$f64$GT$9total_cmp17h9b59c9762e38673bE", scope: !19, file: !18, line: 1345, type: !21, scopeLine: 1345, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !5, templateParams: !28, retainedNodes: !25)
!18 = !DIFile(filename: "/home/tao/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f64.rs", directory: "", checksumkind: CSK_MD5, checksum: "e4fc2bc1adea6d74769e295fcebd34b6")
!19 = !DINamespace(name: "{impl#0}", scope: !20)
!20 = !DINamespace(name: "f64", scope: !11)
!21 = !DISubroutineType(types: !22)
!22 = !{!8, !23, !23}
!23 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&f64", baseType: !24, size: 64, align: 64, dwarfAddressSpace: 0)
!24 = !DIBasicType(name: "f64", size: 64, encoding: DW_ATE_float)
!25 = !{!26, !27}
!26 = !DILocalVariable(name: "self", arg: 1, scope: !17, file: !18, line: 1345, type: !23)
!27 = !DILocalVariable(name: "other", arg: 2, scope: !17, file: !18, line: 1345, type: !23)
!28 = !{}
!29 = !DILocation(line: 1345, column: 22, scope: !17)
!30 = !DILocation(line: 1345, column: 29, scope: !17)
!31 = !DILocation(line: 1346, column: 24, scope: !17)
!32 = !DILocation(line: 1090, column: 18, scope: !33, inlinedAt: !37)
!33 = distinct !DISubprogram(name: "to_bits", linkageName: "_ZN4core3f6421_$LT$impl$u20$f64$GT$7to_bits17h45ba6bb3a41e9fdeE", scope: !19, file: !18, line: 1088, type: !34, scopeLine: 1088, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !5, templateParams: !28)
!34 = !DISubroutineType(types: !35)
!35 = !{!36, !24}
!36 = !DIBasicType(name: "u64", size: 64, encoding: DW_ATE_unsigned)
!37 = !DILocation(line: 1346, column: 29, scope: !17)
!38 = !DILocation(line: 1347, column: 25, scope: !17)
!39 = !DILocation(line: 1090, column: 18, scope: !33, inlinedAt: !40)
!40 = !DILocation(line: 1347, column: 31, scope: !17)
!41 = !DILocation(line: 1371, column: 20, scope: !17)
!42 = !DILocation(line: 1371, column: 19, scope: !17)
!43 = !DILocation(line: 1371, column: 17, scope: !17)
!44 = !DILocation(line: 1371, column: 9, scope: !17)
!45 = !DILocation(line: 1372, column: 21, scope: !17)
!46 = !DILocation(line: 1372, column: 20, scope: !17)
!47 = !DILocation(line: 1372, column: 18, scope: !17)
!48 = !DILocation(line: 1372, column: 9, scope: !17)
!49 = !DILocation(line: 1733, column: 58, scope: !50, inlinedAt: !58)
!50 = distinct !DISubprogram(name: "cmp", linkageName: "_ZN4core3cmp5impls48_$LT$impl$u20$core..cmp..Ord$u20$for$u20$i64$GT$3cmp17hbbc2efb1dbae90dcE", scope: !52, file: !51, line: 1732, type: !54, scopeLine: 1732, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !5, templateParams: !28)
!51 = !DIFile(filename: "/home/tao/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cmp.rs", directory: "", checksumkind: CSK_MD5, checksum: "a6960c98673d991a8260a33ce8a32710")
!52 = !DINamespace(name: "{impl#79}", scope: !53)
!53 = !DINamespace(name: "impls", scope: !10)
!54 = !DISubroutineType(types: !55)
!55 = !{!8, !56, !56}
!56 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&i64", baseType: !57, size: 64, align: 64, dwarfAddressSpace: 0)
!57 = !DIBasicType(name: "i64", size: 64, encoding: DW_ATE_signed)
!58 = !DILocation(line: 1374, column: 14, scope: !17)
!59 = !DILocation(line: 1733, column: 65, scope: !50, inlinedAt: !58)
!60 = !DILocation(line: 1733, column: 21, scope: !50, inlinedAt: !58)
!61 = !DILocation(line: 1375, column: 6, scope: !17)
!62 = distinct !DISubprogram(name: "probe", linkageName: "_ZN26autocfg_7717dfebf98f5077_15probe17hc733925f1f19f1ceE", scope: !64, file: !63, line: 1, type: !65, scopeLine: 1, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !5, templateParams: !28)
!63 = !DIFile(filename: "<anon>", directory: "", checksumkind: CSK_MD5, checksum: "ca821b87a81998bc0a84ab6029e9650c")
!64 = !DINamespace(name: "autocfg_7717dfebf98f5077_1", scope: null)
!65 = !DISubroutineType(types: !66)
!66 = !{null}
!67 = !DILocation(line: 1, column: 26, scope: !62)
!68 = !DILocation(line: 1, column: 50, scope: !69)
!69 = !DILexicalBlockFile(scope: !62, file: !63, discriminator: 0)
