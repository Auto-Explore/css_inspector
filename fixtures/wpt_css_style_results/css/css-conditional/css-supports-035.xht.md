# css/css-conditional/css-supports-035.xht

```json
{
  "format_version": 3,
  "file": "css/css-conditional/css-supports-035.xht"
}
```

## style[0]

```css
<![CDATA[
    html { background-color: green }
    @supports not ({ something @with (unbalanced parens }) {
      html { background-color: red }
    }
    /* parser still looking for second close parenthesis */
    html { background-color: red }
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
