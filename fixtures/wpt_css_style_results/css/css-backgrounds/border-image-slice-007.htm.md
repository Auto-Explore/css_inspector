# css/css-backgrounds/border-image-slice-007.htm

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-image-slice-007.htm"
}
```

## style[0]

```css

            div
            {
                background-color: orange;
                border-color: red;
                border-style: double;
                border-width: 40px;
                border-image-slice: 40% 30% 20% 10% fill;
                /*
                The original test was using 'border-image-slice: 40% 15% 20% 5% fill'.
                */
                border-image-source: url("support/9grid40-30-20-10-green.png");
                /*
                The top side uses 3 blue borders of 24px wide interleaved with 2
                transparent areas of 24px wide.
                So: 5 times 24px == 120px which is 40% of 300px

                The right side uses 3 blue borders of 18px wide interleaved with 2
                transparent areas of 18px wide.
                So: 5 times 18px == 90px which is 30% of 300px

                The bottom side uses 3 blue borders of 4px wide interleaved with 2
                transparent areas of 12px wide.
                So: 5 times 12px == 60px which is 20% of 300px

                The left side uses 3 blue borders of 6px wide interleaved with 2
                transparent areas of 6px wide.
                So: 5 times 6px == 30px which is 10% of 300px
                */
                height: 100px;
                margin: 50px;
                width: 200px;
            }
        
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-image-slice”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
