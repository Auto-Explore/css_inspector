# css/css-exclusions/css3-exclusions/exclusions-wrap-flow-01.xht

```json
{
  "format_version": 3,
  "file": "css/css-exclusions/css3-exclusions/exclusions-wrap-flow-01.xht"
}
```

## style[0]

```css

    /*<![CDATA[*/

#containing-block {
    position: relative;
    width: 400px;
    height: 400px;
    border: 1px solid #e0e0e0;
    background: red;
    font-size: 0px; /* collapses the 'strut' on each line, see CSS 2.1 section 10.8.1 */
}

#exclusion {
    wrap-flow: both;
}

#top-fragment, #left-fragment, #right-fragment, #bottom-fragment {
    display: inline-block;
    background: black;
}

#top-fragment, #bottom-fragment {
    width: 400px;
    height: 150px;
}

#left-fragment, #right-fragment {
    width: 150px;
    height: 100px;
}

#exclusion {
    position: absolute;
    top: 150px;
    left: 150px;
    width: 100px;
    height: 100px;
    background: #0f0;
}


/*]]>*/
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
