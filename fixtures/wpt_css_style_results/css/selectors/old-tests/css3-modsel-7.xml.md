# css/selectors/old-tests/css3-modsel-7.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-7.xml"
}
```

## style[0]

```css
<![CDATA[p { background-color : red }
p[class~="b"] { background-color : lime }
address { background-color : red }
address[title~="foo"] { background-color : lime }
span[class~="b"] { background-color : red }]]>
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
