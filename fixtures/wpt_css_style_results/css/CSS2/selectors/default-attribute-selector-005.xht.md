# css/CSS2/selectors/default-attribute-selector-005.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/selectors/default-attribute-selector-005.xht"
}
```

## style[0]

```css
<![CDATA[
  a {background-color: green;}
  a[shape] {background-color: red;}
  a[shape="rect"] {background-color: green;}
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
