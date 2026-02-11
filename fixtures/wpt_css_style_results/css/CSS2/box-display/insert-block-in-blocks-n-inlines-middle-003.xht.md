# css/CSS2/box-display/insert-block-in-blocks-n-inlines-middle-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/box-display/insert-block-in-blocks-n-inlines-middle-003.xht"
}
```

## style[0]

```css
<![CDATA[
  div.container
  {
  background-color: fuchsia;
  color: black;
  font: 20px/1 Ahem;
  margin: 1em;
  }

  div > div {margin: 1em 0em;}

  div.inserted
  {
  border-left: yellow solid 0.5em;
  border-right: yellow solid 0.5em;
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
