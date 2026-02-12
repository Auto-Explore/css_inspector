# css/css-mixins/mixin-declarations.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/mixin-declarations.html"
}
```

## style[0]

```css

      @mixin --m1() {
        @result {
          color: green;
        }
      }
      div {
        color: red;
        @apply --m1;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
