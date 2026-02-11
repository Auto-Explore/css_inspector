# css/CSS2/css1/c11-import-000.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/css1/c11-import-000.xht"
}
```

## style[0]

```css
<![CDATA[
   @import url(support/a-green.css);
   @import "support/b-green.css";
   .c { color: green; }
   @import url(support/c-red.css);
   <!-- .d { color: green; } -->
  ]]>
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
