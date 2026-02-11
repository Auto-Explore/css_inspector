# css/selectors/old-tests/css3-modsel-17.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-17.xml"
}
```

## style[0]

```css
<![CDATA[p.test a { background-color : red }
p.test *:visited { background-color : lime }]]>
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
