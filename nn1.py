import numpy as np
d = np.loadtxt("append.txt",delimiter=",")
y=d[:, -1]
x =d[:, :-1]
from sklearn.neural_network import MLPRegressor
nn = MLPRegressor(hidden_layer_sizes=(12,12), activation='relu', solver='adam', early_stopping=True,max_iter=500)
n = nn.fit(x, y)
np.savetxt('nnweight1.txt', nn.coefs_[0], delimiter='\n')
np.savetxt('nnweight2.txt', nn.coefs_[1], delimiter='\n')
np.savetxt('nnweight3.txt', nn.coefs_[2], delimiter='\n')
np.savetxt('nnbias1.txt', nn.intercepts_[0], delimiter='\n')
np.savetxt('nnbias2.txt', nn.intercepts_[1], delimiter='\n')
np.savetxt('nnbias3.txt', nn.intercepts_[2], delimiter='\n')
print( nn.score(x,y) )
print(nn.predict([x[0]]))
print(nn.predict([x[1]]))
print(nn.predict([x[2]]))
print(nn.predict([x[3]]))
print(nn.predict([x[4]]))
