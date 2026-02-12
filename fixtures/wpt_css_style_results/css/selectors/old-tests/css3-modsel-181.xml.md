# css/selectors/old-tests/css3-modsel-181.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-181.xml"
}
```

## style[0]

```css
<![CDATA[
 .cs { color: green; }
 .cs P { background: red; color: yellow; }
 .cs .a { background: red; color: yellow; }
 .cs .span1 span { background: red; color: yellow; }
 .cs .span2 { color: red; }
 .cs .span2 SPAN { color: green; }
 .cs .span2 span { background: red; color: yellow; }
 .ci { color: red; }
 .ci P { background: green; color: white; }
 .ci .a { background: green; color: white; }
 .ci .span1 span { background: green; color: white; }
 .ci .span2 SPAN { background: green; color: white; }
]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
