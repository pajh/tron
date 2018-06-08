fmain = open('./src/main.rs', 'r')
main_contents = fmain.read()
fmain.close()

fboard = open("./src/board.rs",'r')
board_contents = fboard.read()
board_contents = "mod board {\n" + board_contents + "} \n"
fboard.close()

fsupport = open("./src/support.rs",'r')
support_contents = fsupport.read()
support_contents = "mod support {\n" + support_contents + "}\n"
fsupport.close()

faimonte = open("./src/aimonte.rs",'r')
aimonte_contents = faimonte.read()
aimonte_contents = "mod aimonte {\n" + aimonte_contents + "}\n"
faimonte.close()

fai = open("./src/ai.rs",'r')
ai_contents = fai.read()
ai_contents = "mod ai {\n" + ai_contents + "}\n"
fai.close()

main_contents = main_contents.replace("mod board;", board_contents )
main_contents = main_contents.replace("mod support;", support_contents )
main_contents = main_contents.replace("mod aimonte;", aimonte_contents )
main_contents = main_contents.replace("mod ai;", ai_contents )

fsubmit = open('./src/submit.rs', 'w')
fsubmit.write(main_contents)
fsubmit.close()

