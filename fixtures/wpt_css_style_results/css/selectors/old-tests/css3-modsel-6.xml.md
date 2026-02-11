# css/selectors/old-tests/css3-modsel-6.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-6.xml"
}
```

## style[0]

```css
<![CDATA[address { background-color : red }
address[title="foo"] { background-color : lime }
span[title="a"] { background-color : red }]]>
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
