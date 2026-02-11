# css/selectors/old-tests/css3-modsel-66.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-66.xml"
}
```

## style[0]

```css
<![CDATA[p { background-color: navy; color: white; }
p:not(:target) { background-color: white; color: black; }]]>
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
