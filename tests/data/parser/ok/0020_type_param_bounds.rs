struct A<T>;
struct B<T:>;
struct C<T: 'a>;
struct D<T: 'a + >;
struct E<T: 'a + 'd >;
struct F<T: 'a + 'd + Clone>;
struct G<T: Clone + Copy>;
struct H<T: ::Foo + self::Bar + 'a>;
struct I<T:, U:,>;