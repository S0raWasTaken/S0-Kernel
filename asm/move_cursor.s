mov dl, 80
mov bx, 24
mov ax, 9
mul dl
add bx, ax

mov dx, 0x03D4
mov al, 0x0F
out dx, al

inc dl
mov al, bl
out dx, al

dec dl
mov al, 0x0E
out dx, al

inc dl
mov al, bh
out dx, al
