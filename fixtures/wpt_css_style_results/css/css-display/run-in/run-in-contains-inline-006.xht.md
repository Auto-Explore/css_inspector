# css/css-display/run-in/run-in-contains-inline-006.xht

```json
{
  "format_version": 3,
  "file": "css/css-display/run-in/run-in-contains-inline-006.xht"
}
```

## style[0]

```css
<![CDATA[
    div { display: block; }
    .run-in { display: run-in; font-weight: bold }
    #target { border: 2px solid black; position: relative; }
    .run-in > span { visibility: hidden; } /* tests that the abs pos actually
                                              shows up */
    .run-in > span > span { position: absolute; visibility: visible; top: 0}
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
