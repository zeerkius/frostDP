# gamblers ruin situation
# made after reading stochastic processes an introudction ~ authors p.w jones & p. smith
# two betters decide to bet , and start with some capital
# they both have some bets in N (set of natural numbers) where n_i is >= 0
# if A bets 1 and B bets 1 if A wins they get 1 dollar and B looses a dollar they win B bet and visa versa

import sys
import threading
import time
import warnings

class Simulation:
    def __init__(self,k , a):
        self.total = a
        if a <= k:
            raise ValueError("new capital must be greater than inital betters capital")
        if k <= 0:
            warnings.warn("\nRun a normal simulation to repay your debt! .simulation() \n" +
                           "run a debted simulation to pay off your debt .debted_simulation() \n" +
                           "& win all of the other parties money\n")

        self.a_start = k
        self.b_start =  (a - k)

    def random_prob(self,p):
        import random
        q = 1 - p
        # 1 -> sucess
        # 0 -> fail
        choice = random.randint(0 , 1)
        if choice <= p:
            return 1
        else:
            return 0

    def simulation(self , a_bet_weight = 1 , b_bet_weight = 1 , a_success_rate = 0.5 , betting_limit = 10000000): # returns winners bet record
        steps = 0
        n = [steps] # time
        a_bank = [self.a_start]
        b_bank = [self.b_start]
        # everything is done from A's perspective howver
        while True:
            if self.a_start == 0 or steps == betting_limit:
                print(" A's ruined!!!" , end = "\n")
                break
            if self.b_start == 0 or steps == betting_limit: # b's ruined
                print(" B's ruined!!!" , end = "\n")
                break
            else:
                if self.random_prob(a_success_rate) == 1:
                    self.a_start += b_bet_weight
                    self.b_start -= b_bet_weight
                if self.random_prob(a_success_rate) == 0:
                    self.b_start += a_bet_weight
                    self.a_start -= a_bet_weight
                a_bank.append(self.a_start)
                b_bank.append(self.b_start)
                steps += 1
                n.append(steps)
        def make_graph(x , y1 , y2):
            import matplotlib.pyplot as plt
            fig , axs = plt.subplots(1,2 , sharey = True , figsize = (10 , 7) , facecolor = "red")
            axs[0].set_title("A's Betting Record")
            axs[1].set_title("B's Betting Record")
            axs[0].set_xlabel("Bets")
            axs[1].set_xlabel("Bets")
            axs[0].set_ylabel("Capital")
            axs[1].set_ylabel("Capital")
            axs[0].plot(x,y1 , color = "red")
            axs[1].plot(x,y2 , color = "blue")
            plt.show()
        make_graph(n,a_bank,b_bank)

        if self.a_start != 0:
            return [n , a_bank]
        else:
            return [n , b_bank]

    def debted_simulation(self , a_bet_weight = 1 , b_bet_weight = 1 , a_success_rate = 0.5 , betting_limit = 10000000): # returns winners bet record
        # A's in debt and decides to bet to see if they wins all of the others parties money
        steps = 0
        n = [steps] # time
        a_bank = [self.a_start]
        b_bank = [self.b_start]
        # everything is done from A's perspective howver
        while True:
            if self.a_start == self.total:
                print("A's Done The Impossible & B's Ruined!!!" , end = "\n")
                break
            else:
                if self.random_prob(a_success_rate) == 1:
                    self.a_start += b_bet_weight
                    self.b_start -= b_bet_weight
                if self.random_prob(a_success_rate) == 0:
                    self.b_start += a_bet_weight
                    self.a_start -= a_bet_weight
                a_bank.append(self.a_start)
                b_bank.append(self.b_start)
                steps += 1
                n.append(steps)
        def make_graph(x , y1 , y2):
            import matplotlib.pyplot as plt
            fig , axs = plt.subplots(1,2 , sharey = True , figsize = (10 , 7) , facecolor = "red")
            axs[0].set_title("A's Betting Record")
            axs[1].set_title("B's Betting Record")
            axs[0].set_xlabel("Bets")
            axs[1].set_xlabel("Bets")
            axs[0].set_ylabel("Capital")
            axs[1].set_ylabel("Capital")
            axs[0].plot(x,y1 , color = "red")
            axs[1].plot(x,y2 , color = "blue")
            plt.show()
        make_graph(n,a_bank,b_bank)
        if self.a_start != 0:
            return [n , a_bank]
        else:
            return [n , b_bank]


def animation(stop_event):
    chars = ["$->" * x  for x in range(10)]
    start = 0
    while not stop_event.is_set():
        sys.stdout.write("\r" + chars[start % len(chars)])
        sys.stdout.flush()
        start += 1
        time.sleep(0.2)

sim = Simulation(300,1000) # meaning we have a bet where total capital is 300 dolars A -> starts with 100 , B -> starts with 200
# we can change the probability of A winning as well as the stakes of how much either looses or wins


# to start animation as well as simulation we define entry point


if __name__ == "__main__":
    #stop_event = threading.Event() # - > so this is an event for each what would be thread to execture the animation
    #animation_thread = threading.Thread(target=animation,args=(stop_event,)) # the comma signifies tuples
    #animation_thread.start() # start thread
    sim.debted_simulation(a_success_rate= 0.8, a_bet_weight= 1, b_bet_weight=1) # run clutsering
    #stop_event.set() # so this allows for the little process of animation to animate , then stop
    #animation_thread.join()
    #print("Simulation Over")









