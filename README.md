# trait objects

有关trait 和 trait objects的使用.

## trait_object_in_fn.rs
在参数中使用`trait object`的常用三种方法
- &impl Trait
- &dyn Trait
- Box<dyn Trait>

看一个实战中的例子:
```rust
pub trait CookieStore: Send + Sync {
    fn set_cookies(
     &self,
     cookie_headers: &mut dyn Iterator<Item = &HeaderValue>,
     url: &Url,
    );
    
    fn cookies(&self, url: &Url) -> Option<HeaderValue>;
}
```
这是我们之前使用过的**reqwest**库中的一个处理`CookieStore`的`trait`。在`set_cookies`方法中使用了`&mut dyn Iterator`这样一个`trait object`。

## service.rs
在函数返回值中使用`trait object`

为何 KV server 里的 Storage trait 不能使用泛型参数来处理返回的 iterator，只能用`Box<dyn Iterator>`：

```rust
pub trait Storage: Send + Sync + 'static {
    /// 遍历HashTable, 返回 kv pair 的 Iterator
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>>;
}
```

再看一个CryptoResolved的例子:
```rust
/// An object that resolves the providers of Noise crypto choices
pub trait CryptoResolver {
    /// Provide an implementation of the Random trait or None if none available.
    fn resolve_rng(&self) -> Option<Box<dyn Random>>;

    /// Provide an implementation of the Dh trait for the given DHChoice or None if unavailable.
    fn resolve_dh(&self, choice: &DHChoice) -> Option<Box<dyn Dh>>;

    /// Provide an implementation of the Hash trait for the given HashChoice or None if unavailable.
    fn resolve_hash(&self, choice: &HashChoice) -> Option<Box<dyn Hash>>;

    /// Provide an implementation of the Cipher trait for the given CipherChoice or None if unavailable.
    fn resolve_cipher(&self, choice: &CipherChoice) -> Option<Box<dyn Cipher>>;

    /// Provide an implementation of the Kem trait for the given KemChoice or None if unavailable
    #[cfg(feature = "hfs")]
    fn resolve_kem(&self, _choice: &KemChoice) -> Option<Box<dyn Kem>> {
        None
    }
}
```

这是一个处理 Noise Protocol 使用何种加密算法的一个 trait。这个 trait 的每个方法，都返回一个 trait object，每个 trait object 都提供加密算法中所需要的不同的能力，比如随机数生成算法（Random）、DH 算法（Dh）、哈希算法（Hash）、对称加密算法（Cipher）和密钥封装算法（Kem）。
