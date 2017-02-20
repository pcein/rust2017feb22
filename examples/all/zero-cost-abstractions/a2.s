	.text
	.file	"a2.0.rs"
	.section	.text._ZN2a24main17h5709137d6dd5483bE,"ax",@progbits
	.align	16, 0x90
	.type	_ZN2a24main17h5709137d6dd5483bE,@function
_ZN2a24main17h5709137d6dd5483bE:
	.cfi_startproc
	subq	$72, %rsp
.Ltmp0:
	.cfi_def_cfa_offset 80
	movabsq	$4999999950000000, %rax
	movq	%rax, 64(%rsp)
	movq	_ZN4core3fmt3num46_$LT$impl$u20$fmt..Display$u20$for$u20$u64$GT$3fmt17hb40947e773a6f128E@GOTPCREL(%rip), %rax
	movq	%rax, 8(%rsp)
	leaq	64(%rsp), %rax
	movq	%rax, (%rsp)
	leaq	ref4681(%rip), %rax
	movq	%rax, 16(%rsp)
	movq	$2, 24(%rsp)
	xorps	%xmm0, %xmm0
	movups	%xmm0, 32(%rsp)
	leaq	(%rsp), %rax
	movq	%rax, 48(%rsp)
	movq	$1, 56(%rsp)
	leaq	16(%rsp), %rdi
	callq	_ZN3std2io5stdio6_print17h03730948b3f63a9bE@PLT
	addq	$72, %rsp
	retq
.Lfunc_end0:
	.size	_ZN2a24main17h5709137d6dd5483bE, .Lfunc_end0-_ZN2a24main17h5709137d6dd5483bE
	.cfi_endproc

	.section	.text.main,"ax",@progbits
	.globl	main
	.align	16, 0x90
	.type	main,@function
main:
	.cfi_startproc
	movq	%rsi, %rax
	movq	%rdi, %rcx
	leaq	_ZN2a24main17h5709137d6dd5483bE(%rip), %rdi
	movq	%rcx, %rsi
	movq	%rax, %rdx
	jmp	_ZN3std2rt10lang_start17h5b0863080165c75eE@PLT
.Lfunc_end1:
	.size	main, .Lfunc_end1-main
	.cfi_endproc

	.type	str4679,@object
	.section	.rodata.str4679,"a",@progbits
str4679:
	.size	str4679, 0

	.type	str4680,@object
	.section	.rodata.str4680,"a",@progbits
str4680:
	.byte	10
	.size	str4680, 1

	.type	ref4681,@object
	.section	.data.rel.ro.ref4681,"aw",@progbits
	.align	8
ref4681:
	.quad	str4679
	.quad	0
	.quad	str4680
	.quad	1
	.size	ref4681, 32


	.section	".note.GNU-stack","",@progbits
