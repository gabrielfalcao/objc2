use objc2::mutability::CounterpartOrSelf;
use objc2::rc::Id;
use objc2::runtime::NSZone;
use objc2::{extern_protocol, ProtocolType};

extern_protocol!(
    /// A protocol to provide functional copies of objects.
    ///
    /// This is similar to Rust's [`Clone`] trait, along with sharing a few
    /// similarities to the [`std::borrow::ToOwned`] trait with regards to the
    /// output type.
    ///
    /// See [Apple's documentation][apple-doc] for details.
    ///
    /// [apple-doc]: https://developer.apple.com/documentation/foundation/nscopying
    pub unsafe trait NSCopying {
        /// Returns a new instance that's a copy of the receiver.
        ///
        /// The output type is the immutable counterpart of the object, which is
        /// usually `Self`, but e.g. `NSMutableString` returns `NSString`.
        #[method_id(copy)]
        #[optional]
        fn copy(&self) -> Id<Self::Immutable>
        where
            Self: CounterpartOrSelf;

        /// Returns a new instance that's a copy of the receiver.
        ///
        /// This is only used when implementing `NSCopying`, use
        /// [`copy`][NSCopying::copy] instead.
        ///
        ///
        /// # Safety
        ///
        /// The zone pointer must be valid or NULL.
        #[method_id(copyWithZone:)]
        unsafe fn copyWithZone(&self, zone: *mut NSZone) -> Id<Self::Immutable>
        where
            Self: CounterpartOrSelf;
    }

    unsafe impl ProtocolType for dyn NSCopying {}
);

// FIXME: Remove this hack which makes NSMutableDictionary tests work
unsafe impl NSCopying for objc2::rc::__RcTestObject {}

extern_protocol!(
    /// A protocol to provide mutable copies of objects.
    ///
    /// Only classes that have an “immutable vs. mutable” distinction should adopt
    /// this protocol.
    ///
    /// See [Apple's documentation][apple-doc] for details.
    ///
    /// [apple-doc]: https://developer.apple.com/documentation/foundation/nsmutablecopying
    pub unsafe trait NSMutableCopying {
        /// Returns a new instance that's a mutable copy of the receiver.
        ///
        /// The output type is the mutable counterpart of the object. E.g. both
        /// `NSString` and `NSMutableString` return `NSMutableString`.
        #[method_id(mutableCopy)]
        #[optional]
        fn mutableCopy(&self) -> Id<Self::Mutable>
        where
            Self: CounterpartOrSelf;

        /// Returns a new instance that's a mutable copy of the receiver.
        ///
        /// This is only used when implementing `NSMutableCopying`, use
        /// [`mutableCopy`][NSMutableCopying::mutableCopy] instead.
        ///
        ///
        /// # Safety
        ///
        /// The zone pointer must be valid or NULL.
        #[method_id(mutableCopyWithZone:)]
        unsafe fn mutableCopyWithZone(&self, zone: *mut NSZone) -> Id<Self::Mutable>
        where
            Self: CounterpartOrSelf;
    }

    unsafe impl ProtocolType for dyn NSMutableCopying {}
);
