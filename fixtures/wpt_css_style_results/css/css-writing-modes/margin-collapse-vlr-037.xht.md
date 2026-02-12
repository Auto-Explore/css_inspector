# css/css-writing-modes/margin-collapse-vlr-037.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/margin-collapse-vlr-037.xht"
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
      background: red url("support/margin-collapse-2em-space-wm-vert.png") -1em top;
      border-left: green solid 1em;
      width: 3em;
      writing-mode: vertical-lr;
    }

  div#parent
    {
      background-color: green;
      margin-right: 2em; /* block-end margin */
    }

  div#last-child
    {
      margin-right: 2em; /* block-end margin of last child */
    }

  div#following-parent
    {
      background-color: green;
      width: 1em;
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
