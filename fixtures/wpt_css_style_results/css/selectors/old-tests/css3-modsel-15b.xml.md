# css/selectors/old-tests/css3-modsel-15b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-15b.xml"
}
```

## style[0]

```css
<![CDATA[
p { background: green; color: white; }
#test#fail { background: red; color: yellow; }
#fail#test { background: red; color: yellow; }
#fail { background: red; color: yellow; }
div { background: red; color: yellow; }
#pass#pass { background: green; color: white; }
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
