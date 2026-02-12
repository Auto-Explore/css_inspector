# css/css-writing-modes/sizing-orthog-htb-in-vrl-013-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthog-htb-in-vrl-013-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      direction: rtl;
      width: calc(52px + 100vw + 52px);
    }

  body
    {
      font-size: 16px;
      line-height: 1.25; /* therefore, each line box is 20px tall */
      margin: 8px 0px;
    }

  table
    {
      border-spacing: 0px;
      direction: ltr;
      position: absolute;
      width: calc(52px + 100% + 52px);
    }

  td
    {
      padding: 0px;
      vertical-align: top;
    }

  td#after, td#before
    {
      width: 52px;
    }

  p#sentence-after, p#sentence-before
    {
      writing-mode: vertical-rl;
    }

  td#data
    {
      border: blue solid 3px;
      display: block;
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
