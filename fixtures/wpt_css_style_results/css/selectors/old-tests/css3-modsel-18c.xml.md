# css/selectors/old-tests/css3-modsel-18c.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-18c.xml"
}
```

## style[0]

```css
<![CDATA[
:link, :visited { color: navy; text-decoration: none; }
:link:hover span { background-color : lime }
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
