# css/css-display/run-in/run-in-contains-inline-007.xht

```json
{
  "format_version": 3,
  "file": "css/css-display/run-in/run-in-contains-inline-007.xht"
}
```

## style[0]

```css
<![CDATA[
    div { display: block; }
    .run-in { display: run-in; font-weight: bold }
    #target { border: 2px solid black; clear: both; }
    .run-in > span { visibility: hidden; } /* tests that the float actually
                                              shows up */
    .run-in > span > span { float: left; visibility: visible; }
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
