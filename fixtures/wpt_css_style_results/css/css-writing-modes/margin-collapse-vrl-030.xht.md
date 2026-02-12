# css/css-writing-modes/margin-collapse-vrl-030.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/margin-collapse-vrl-030.xht"
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
      background-image: url("support/margin-collapse-2em-space-wm-vert.png");
      writing-mode: vertical-rl;
    }

  div#right-most-sibling
    {
      background-color: green;
      margin-left: 2em;
      width: 1em;
    }

  div#middle-sibling
    {
      margin-left: 2em;
      margin-right: 2em;
    }

  div#left-most-sibling
    {
      background-color: green;
      margin-right: 2em;
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
