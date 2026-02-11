# css/selectors/old-tests/css3-modsel-14.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-14.xml"
}
```

## style[0]

```css
<![CDATA[p { background-color : red ; border : thick solid red ; padding : 1em }
p.t1 { background-color : lime }
p.t2 { border : thick solid green }

div { background: green; color: white; }
div.teST { background: red; color: yellow; }
div.te { background: red; color: yellow; }
div.st { background: red; color: yellow; }
div.te.st { background: red; color: yellow; }]]>
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
