# css/css-writing-modes/inline-block-alignment-slr-009-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/inline-block-alignment-slr-009-ref.xht"
}
```

## style[0]

```css
<![CDATA[
 img
 {
  padding-left: 252px; /* 60px (padding-left) + 120px (B) + 96px (?) - 24px (?) == 252px */
  vertical-align: top;
 }

 img + br + img
 {
  padding-left: 60px; /* 60px (padding-left) */
 }

 img + br + img + br + img
 {
  padding-left: 228px; /* 60px (padding-left) + 120px (B) + 96px (?) - 48px == 228px */
 }
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
