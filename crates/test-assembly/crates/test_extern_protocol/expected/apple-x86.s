	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_protocol
	.p2align	4, 0x90
_get_protocol:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	call	L0$pb
L0$pb:
	pop	eax
	sub	esp, 8
	lea	eax, [eax + l_anon.[ID].0-L0$pb]
	push	10
	push	eax
	call	SYM(objc2::runtime::Protocol::get::GENERATED_ID, 0)
	add	esp, 24
	pop	ebp
	ret

	.globl	_dyn_call
	.p2align	4, 0x90
_dyn_call:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	call	L1$pb
L1$pb:
	pop	eax
	sub	esp, 8
	push	dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_016b57e4e6a36961-L1$pb]
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 24
	pop	ebp
	ret

	.globl	_dyn_consume
	.p2align	4, 0x90
_dyn_consume:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	call	L2$pb
L2$pb:
	pop	eax
	sub	esp, 8
	push	dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_016b57e4e6a36961-L2$pb]
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 24
	pop	ebp
	jmp	_objc_release

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"MyProtocol"

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_016b57e4e6a36961
	.p2align	2
L_OBJC_IMAGE_INFO_016b57e4e6a36961:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_016b57e4e6a36961
L_OBJC_METH_VAR_NAME_016b57e4e6a36961:
	.asciz	"aMethod"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_016b57e4e6a36961
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_016b57e4e6a36961:
	.long	L_OBJC_METH_VAR_NAME_016b57e4e6a36961

.subsections_via_symbols
