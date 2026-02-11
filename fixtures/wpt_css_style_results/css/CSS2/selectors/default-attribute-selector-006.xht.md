# css/CSS2/selectors/default-attribute-selector-006.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/selectors/default-attribute-selector-006.xht"
}
```

## style[0]

```css
<![CDATA[
  form {background-color: green;}
  form[method] {background-color: red;}
  form[method="get"] {background-color: green;}
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
