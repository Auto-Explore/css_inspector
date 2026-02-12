# css/css-mixins/mixin-from-import.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/mixin-from-import.html"
}
```

## style[0]

```css

      @import 'resources/imported-sheet-with-mixin.css';
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
