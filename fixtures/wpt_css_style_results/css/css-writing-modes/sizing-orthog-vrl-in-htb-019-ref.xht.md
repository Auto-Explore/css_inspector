# css/css-writing-modes/sizing-orthog-vrl-in-htb-019-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthog-vrl-in-htb-019-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      writing-mode: vertical-rl;
    }

  body
    {
      font-size: 16px;
      line-height: 1.25; /* therefore, each line box is 20px tall */
    }

  p
    {
      left: 8px;
      position: absolute;
      writing-mode: horizontal-tb;
    }

  p#sentence-before
    {
      margin-top: 8px;
    }

  div
    {
      border: blue solid 3px;
      height: 394px;
      left: 8px;
      position: absolute;
      top: 52px;
    }

  p#sentence-after
    {
      padding-bottom: 1em;
      top: 452px;
      /*
        52px :
     +
         3px :
     +
       394px :
     +
         3px :
    ============
       452px
      */
    }

  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
