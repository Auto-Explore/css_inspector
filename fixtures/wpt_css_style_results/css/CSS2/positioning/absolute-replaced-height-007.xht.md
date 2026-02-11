# css/CSS2/positioning/absolute-replaced-height-007.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-replaced-height-007.xht"
}
```

## style[0]

```css

            #div1
            {
                position: relative;
            }
            div div
            {
                border: solid green;
                position: absolute;
                width: 300px;
            }
            iframe
            {
                border: solid red;
                position: absolute;
                width: auto;
            }

            /*
            "
            (...)
            the height of the containing block of an absolutely positioned
            element is independent of the size of the element itself, and thus
            a percentage height on such an element *_can always be resolved_*.
            However, it may be that the height is not known until elements
            that come later in the document have been processed. "
            http://www.w3.org/TR/CSS21/visudet.html#the-height-property
            "
            */
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
