# css/css-writing-modes/margin-collapse-vlr-009.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/margin-collapse-vlr-009.xht"
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
      background: red url("support/margin-collapse-2em-space-wm-vert.png");
      width: 4em;
      writing-mode: vertical-lr;
    }

  div#leftmost
    {
      background-color: green;
      margin-right: 2em; /* block-end margin of sub-block of 1st block */
      width: 1em;
    }

  div#rightmost
    {
      background-color: green;
      margin-left: 1em; /* block-start margin of following sibling block */
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
