# css/css-mixins/apply-top-level.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/apply-top-level.html"
}
```

## style[0]

```css

      .cls {
        color: green;
      }
      @mixin --m1() {
        @result {
          .cls {
            color: red;
          }
        }
      }
      @apply --m1;
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
