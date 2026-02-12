# css/css-conditional/css-supports-043.xht

```json
{
  "format_version": 3,
  "file": "css/css-conditional/css-supports-043.xht"
}
```

## style[0]

```css
<![CDATA[
    html { background-color: red; }
    @supports not (color: green !bogus) {
      html { background-color: green; }
    }
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
