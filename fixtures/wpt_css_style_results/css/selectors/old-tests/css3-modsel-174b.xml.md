# css/selectors/old-tests/css3-modsel-174b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-174b.xml"
}
```

## style[0]

```css
<![CDATA[
 tests, tests * { display: block; color: green }
 testA:not([*|attribute="pass"]) { color: red; }
 testB:not([*|attribute="pass"]) { color: red; }
]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
