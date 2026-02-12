# css/selectors/old-tests/css3-modsel-178.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-178.xml"
}
```

## style[0]

```css
<![CDATA[
 div { color: green; }
 p:not(:first-line) { color: yellow; background: red; }
 p:not(:after) { color: yellow; background: red; content: ' THIS TEST HAS FAILED! '; }
]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
