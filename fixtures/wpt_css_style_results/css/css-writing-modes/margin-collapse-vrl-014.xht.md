# css/css-writing-modes/margin-collapse-vrl-014.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/margin-collapse-vrl-014.xht"
}
```

## style[0]

```css
<![CDATA[
  div
    {
      font: 25px/1em Ahem;
      height: 4em;
    }

  div#wrapper
    {
      background: url("support/margin-collapse-2em-space-wm-vert.png") left top;
      border-right: green solid 1em;
      writing-mode: vertical-rl;
    }

  div#wrapper div
    {
      width: 1em;
    }

  div#overflow
    {
      margin-right: 2em; /* block-start margin of overflowed block */
      overflow: visible;
    }

  div#nested
    {
      background-color: green;
      margin-right: 2em; /* block-start margin of sub-block */
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
