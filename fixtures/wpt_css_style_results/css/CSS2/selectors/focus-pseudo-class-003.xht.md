# css/CSS2/selectors/focus-pseudo-class-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/selectors/focus-pseudo-class-003.xht"
}
```

## style[0]

```css
<![CDATA[
  input[type="radio"]
  {
  height: 20px;
  width: 60px;
  }

  input[type="radio"]:focus + label
  {
  background-color: yellow;
  color: black;
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
