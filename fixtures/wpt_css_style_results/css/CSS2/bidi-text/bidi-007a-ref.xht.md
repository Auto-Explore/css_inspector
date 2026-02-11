# css/CSS2/bidi-text/bidi-007a-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/bidi-text/bidi-007a-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  div > p
  {
  background-color: black;
  clear: left;
  color: yellow;
  float: left;
  font: 2em/1 serif;
  margin: 0.5em 1em;
  padding: 0.75em;
  white-space: pre;
  }

  .c, .j, .e {color: aqua;}

  .b, .d, .i, .k {color: fuchsia;}

  .b, .c
  {
  border-style: solid none solid solid;
  padding: 0.1em 0 0.1em 0;
  margin-left: 0.5em;
  }

  .j, .k
  {
  border-style: solid solid solid none;
  padding: 0.1em 0 0.1em 0;
  margin-right: 0.5em;
  }

  .d, .e, .i
  {
  border-style: solid none solid none;
  padding: 0.1em 0 0.1em 0;
  }
  ]]>
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
