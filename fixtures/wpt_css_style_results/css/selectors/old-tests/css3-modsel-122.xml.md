# css/selectors/old-tests/css3-modsel-122.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-122.xml"
}
```

## style[0]

```css
<![CDATA[@namespace html url(http://www.w3.org/1999/xhtml);
@namespace a url(http://www.example.org/a);
@namespace b url(http://www.example.org/b);
div.stub > *|* { background-color : lime ; display : block ;
                 margin-bottom : 1em }
div.stub > *|*:not(*|*) { background-color : red }
/* yes, the rule just above selects nothing... That's the point */
]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
