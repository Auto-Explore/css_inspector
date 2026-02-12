# css/css-mixins/apply-within-mixin.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/apply-within-mixin.html"
}
```

## style[0]

```css

      @mixin --m1() {
        @result {
          &.a {
            @apply --m2;
          }
        }
      }
      .c {
        @apply --m1;
      }
      @mixin --m2() {
        @result {
          &.b {
            color: green;
          }
        }
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
