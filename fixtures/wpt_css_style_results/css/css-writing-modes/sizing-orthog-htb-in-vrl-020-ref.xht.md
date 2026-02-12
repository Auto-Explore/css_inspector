# css/css-writing-modes/sizing-orthog-htb-in-vrl-020-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthog-htb-in-vrl-020-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  body
    {
      font-size: 16px;
      font-family: monospace;
      line-height: 1.25; /* therefore, each line box is 20px tall */
      margin: 8px 0px;
    }

  table
    {
      border-spacing: 0px;
      left: calc(100% - 52px - 3px - 50ch - 3px - 52px);
      /* or more elegantly right: 0px; Chrome 57+ has a bug here */
      position: absolute;
      width: calc(52px + 3px + 50ch + 3px + 52px);
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
