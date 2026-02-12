# css/css-scroll-snap/support/scroll-target-align-001-iframe.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/support/scroll-target-align-001-iframe.html"
}
```

## style[0]

```css

  html, body {
    margin: 0; padding: 0;
  }
  html {
    /* to make failing more obvious */
    background: 0 1em / 100% 1em linear-gradient(red, red) repeat-x fixed;
    /* avoid anti-aliasing issues */
    font: 20px/1 sans-serif;
    scrollbar-width: none;
  }
  div {
    height: 1em;
  }
  html    { scroll-padding:   .5em 0 0; } /* set up a snap position      */
  #target { scroll-margin:    .5em 0 0;
            scroll-snap-align:  center; }
  #stripe { background: green;          } /* color part of the snap area */
  .fail   { color: red;                 } /* make failing more obvious   */

  /* emulate `scrollbar-width: none` for browsers that don't support it yet */
  ::-webkit-scrollbar { display: none; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
