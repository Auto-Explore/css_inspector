# css/css-writing-modes/clearance-calculations-vrl-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/clearance-calculations-vrl-002.xht"
}
```

## style[0]

```css
<![CDATA[
  div
    {
      height: 5em;
    }

  div#parent
    {
      background: red url("support/clearance-calculation-vrl-002.png");
      font: 20px/1 Ahem; /* computes to 20px/20px */
      min-width: 6em;
      writing-mode: vertical-rl;
    }

  div > div
    {
      background-color: green;
      width: 1em;
    }

  div#preceding-sibling
    {
      margin-left: 1em;
    }

  div#floated-left
    {
      float: left;
    }

  div#clearing-left
    {
      clear: left;
      margin-right: 4em;
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
