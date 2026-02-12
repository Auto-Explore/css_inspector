# css/css-writing-modes/block-flow-direction-vrl-021.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/block-flow-direction-vrl-021.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      writing-mode: vertical-rl;
    }
  /*
  "
  The principal writing mode of the document is determined by the writing-mode
  and direction values specified on the root element.
  "
  */

  body
    {
      color: yellow;
      font: 20px/1 Ahem;
      height: 9em;
    }

  ul
    {
      background-color: blue;
      border-bottom: blue solid 1em;
      list-style: none outside url("support/blue1x1.png");
      margin: 0em;
      padding-top: 1em; /* overriding default padding-start: 40px in several browsers */
    }

  ul.right-border
    {
      border-right: blue solid 1em;
    }

  ul#left-border
    {
      border-left: blue solid 1em;
    }

  /*
     This test depends on the blue ::marker image being placed inside the (blue)
     padding area.  That depends on how the spacing between it and the list-item
     box is calculated, which depends on font metrics.  The following rule is
     to avoid these uncertainties and place it inside the padding for sure.
  */
  ::marker
    {
      font-size: 0;
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
