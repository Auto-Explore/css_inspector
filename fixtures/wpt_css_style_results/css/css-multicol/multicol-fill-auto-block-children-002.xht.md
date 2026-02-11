# css/css-multicol/multicol-fill-auto-block-children-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-fill-auto-block-children-002.xht"
}
```

## style[0]

```css
<![CDATA[
  html {background-color: white;}

  body
  {
  background-color: blue;
  height: 200px;
  margin: 8px;
  width: 60%;

  column-count: 3;
  column-fill: auto;
  column-gap: 10px;
  }

  h1
  {
  color: white;
  column-span: all;
  font-size: 2em;
  line-height: 1.25; /* or 1.21875 to achieve a 39px tall line box */
  margin: 21px 0em;
  /*
   21px : margin-top of h1 element which must not collapse with body's margin-top
   80px : content height: 2 line boxes required to render the "Test passes if ..." sentence
   21px : margin-bottom of h1 element
 ====================================
  122px : margin box height of h1 element
  */
  }

  h2
  {
  color: blue;
  font-size: 1.5em;
  line-height: 1;
  margin: 0 0 2.25em;
  /*
    0px : margin-top of h2 element
   24px : content height: 1 line box required to render all the nbsp; and PASS! word
   54px : margin-bottom of h2 element
 ====================================
   78px : margin box height of h2 element
  */
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
