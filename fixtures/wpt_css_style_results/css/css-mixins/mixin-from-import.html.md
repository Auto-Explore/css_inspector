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
  "errors": 2,
  "messages": [
    {
      "message": "Imported style sheets are not checked.",
      "severity": "Warning"
    },
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 1
}
```
